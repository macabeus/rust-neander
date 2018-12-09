use state::State;
use state::operator::Operator;
use state::memory_line::LineKind;
use state::memory_line::MemoryLine;
use ui::uistate::BlockLists;
use ui::uistate::ListState;
use ui::uistate::UIState;
use ui::list_action::ListActions;
use ui::tui::backend::Backend;
use ui::tui::widgets::{Widget, Block, Borders, Text, List};
use ui::tui::layout::Rect;
use ui::tui::Frame;

pub enum MemoryListKind {
    Operators,
    Variables,
}

fn format_operator_line(position: usize, operator: &Operator, operator_memory: &u8) -> String {
    format!(
        "{:#04X}: {:?}  {:#04X}",
        position, operator.mnemonic, operator_memory,
    )
}

fn format_argument_line(position: usize, argument: &u8) -> String {
    format!(
        "{:#04X}:           {:#04X}",
        position, argument,
    )
}

fn format_variable_line(position: usize, value: &u8) -> String {
    format!(
        "{:#04X}: {:#04X}",
        position, value,
    )
}

fn format_typing_operator_line(position: usize, typing: char) -> String {
    format!(
        "{:#04X}:      0x{}_",
        position, typing,
    )
}

fn format_typing_argument_line(position: usize, typing: char) -> String {
    format!(
        "{:#04X}:           0x{}_",
        position, typing,
    )
}

fn format_typing_variable_line(position: usize, typing: char) -> String {
    format!(
        "{:#04X}: 0x{}_",
        position, typing,
    )
}

fn add_selected_line_arrow(
    uistate: &UIState,
    memory_list_kind: &MemoryListKind,
    position: usize,
    line: String
) -> String {
    if position != uistate.current_list().current_line {
        return format!("    {}", line)
    }

    match (&uistate.block_selected, memory_list_kind) {
        (BlockLists::Operators, MemoryListKind::Operators) => format!(" -> {}", line),
        (BlockLists::Variables, MemoryListKind::Variables) => format!(" -> {}", line),
        _ => format!("    {}", line),
    }
}

fn format_memory(
    uistate: &UIState,
    state: &State,
    memory_line: &MemoryLine,
    position: usize,
) -> String {
    let is_the_selected_line = match uistate.block_selected {
        BlockLists::Operators => position == uistate.current_list().current_line,
        _ => false,
    };
    let is_the_running_line = position == state.pc;

    let line = match (memory_line.kind, is_the_selected_line, uistate.is_typing) {
        (LineKind::Operator, true, true) => format_typing_operator_line(position, uistate.typing_char.unwrap()),
        (LineKind::Argument, true, true) => format_typing_argument_line(position, uistate.typing_char.unwrap()),
        (LineKind::Operator, _, _) => format_operator_line(position, &memory_line.operator, &memory_line.value),
        (LineKind::Argument, _, _) => format_argument_line(position, &memory_line.value),
    };

    if is_the_running_line {
        format!("{} (PC)", line)
    } else {
        line
    }
}

fn format_variable(
    uistate: &UIState,
    memory_line: &MemoryLine,
    position: usize,
) -> String {
    let is_the_selected_line = match uistate.block_selected {
        BlockLists::Variables => position == uistate.current_list().current_line,
        _ => false,
    };

    match (is_the_selected_line, uistate.is_typing) {
        (true, true) => format_typing_variable_line(position, uistate.typing_char.unwrap()),
        (_, _) => format_variable_line(position, &memory_line.value),
    }
}

fn add_comment(
    state: &State,
    position: usize,
    line: String
) -> String {
    let comment = state.comments[position];
    if comment.chars().count() > 0 {
        format!("{} ; {}", line, comment)
    } else {
        line
    }
}

pub fn draw<B>(
    uistate: &UIState,
    current_state: &State,
    memory_list_kind: MemoryListKind,
    f: &mut Frame<B>,
    area: Rect
) where B: Backend,
{
    let memory_list_state: &ListState;
    let format: Box<Fn((usize, &MemoryLine)) -> String>;
    match memory_list_kind {
        MemoryListKind::Operators => {
            memory_list_state = &uistate.memory_list_operators;
            format = Box::new(|(i, memory_line)| {
                format_memory(
                    uistate,
                    current_state,
                    memory_line,
                    i + memory_list_state.first_line,
                )
            })
        },

        MemoryListKind::Variables => {
            memory_list_state = &uistate.memory_list_variables;
            format = Box::new(|(i, memory_line)| {
                format_variable(
                    uistate,
                    memory_line,
                    i + memory_list_state.first_line,
                )
            })
        },
    }

    let list_operators = current_state.list_operators();
    let list_operators_slice = &list_operators[memory_list_state.first_line..=memory_list_state.last_line];

    let memory_str_table = list_operators_slice
        .iter()
        .enumerate()
        .map(|i| format(i))
        .enumerate()
        .map(|(i, line)|
             add_selected_line_arrow(
                 &uistate,
                 &memory_list_kind,
                 i + memory_list_state.first_line,
                 line,
             )
        )
        .enumerate()
        .map(|(i, line)|
             add_comment(
                 &current_state,
                 i + memory_list_state.first_line,
                 line,
             )
        )
        .map(Text::raw);

    List::new(memory_str_table)
      .block(Block::default().borders(Borders::ALL).title(" Memory "))
      .render(f, area);
}

// Actions
pub const MEMORY_LIST_ACTIONS: ListActions = ListActions {
    move_up_cursor_handle: |list_state: &mut ListState| {
        if list_state.current_line == 0 {
            return;
        }

        list_state.current_line -= 1;

        if list_state.first_line + 3 > list_state.current_line && list_state.first_line > 0 {
            list_state.first_line -= 1;
            list_state.last_line -= 1;
        }
    },

    move_down_cursor_handle: |list_state: &mut ListState| {
        if list_state.current_line == 0xFF {
            return;
        }

        list_state.current_line += 1;

        if list_state.last_line - 3 < list_state.current_line && list_state.last_line < 0xFF {
            list_state.first_line += 1;
            list_state.last_line += 1;
        }
    },

    set_type_u8_handle: |state: &mut State, line_number: usize, u8_value: u8| {
        state.memory[line_number] = u8_value;
    },

    select_line_handle: |state: &mut State, line_number: usize| {
        state.pc = line_number;
    },
};
