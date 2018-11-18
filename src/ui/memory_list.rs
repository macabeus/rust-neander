use state::State;
use state::operator::Operator;
use state::memory_line::LineKind;
use state::memory_line::MemoryLine;
use ui::uistate::UIState;
use ui::tui::backend::Backend;
use ui::tui::widgets::{Widget, Block, Borders, Text, List};
use ui::tui::layout::Rect;
use ui::tui::Frame;

fn format_operator_line(position: &usize, operator: &Operator, operator_memory: &u8) -> String {
    format!(
        "{:#04X}: {:?}  {:#04X}",
        position, operator.mnemonic, operator_memory,
    )
}

fn format_argument_line(position: &usize, argument: &u8) -> String {
    format!(
        "{:#04X}:           {:#04X}",
        position, argument,
    )
}

fn format_typing_operator_line(position: &usize, typing: char) -> String {
    format!(
        "{:#04X}:      0x{}_",
        position, typing,
    )
}

fn format_typing_argument_line(position: &usize, typing: char) -> String {
    format!(
        "{:#04X}:           0x{}_",
        position, typing,
    )
}

fn format_memory_line(
    uistate: &UIState,
    memory_line: &MemoryLine,
    position: &usize,
) -> String {
    let is_current_line = *position == uistate.current_line;

    let line = match (memory_line.kind, is_current_line, uistate.is_typing) {
        (LineKind::Operator, true, true) => format_typing_operator_line(position, uistate.typing_char.unwrap()),
        (LineKind::Argument, true, true) => format_typing_argument_line(position, uistate.typing_char.unwrap()),
        (LineKind::Operator, _, _) => format_operator_line(position, &memory_line.operator, &memory_line.value),
        (LineKind::Argument, _, _) => format_argument_line(position, &memory_line.value),
    };

    if is_current_line {
        format!(" -> {}", line)
    } else {
        format!("    {}", line)
    }
}

pub fn draw<B>(uistate: &UIState, current_state: &State, f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let list_operators = current_state.list_operators();
    let list_operators_slice = &list_operators[uistate.memory_list_first_line..=uistate.memory_list_last_line];

    let memory_str_table = list_operators_slice
        .iter()
        .enumerate()
        .map(|(i, memory_line)|
            Text::raw(
                format_memory_line(
                    uistate,
                    memory_line,
                    &(i + uistate.memory_list_first_line),
                )
            )
        );

    List::new(memory_str_table)
      .block(Block::default().borders(Borders::ALL).title(" Memory "))
      .render(f, area);
}
