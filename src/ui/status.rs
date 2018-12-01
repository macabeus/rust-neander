use state::State;
use ui::uistate::UIState;
use ui::uistate::BlockLists;
use ui::uistate::ListState;
use ui::list_action::ListActions;
use ui::tui::backend::Backend;
use ui::tui::widgets::{Widget, Block, Borders, List, Text};
use ui::tui::layout::Rect;
use ui::tui::Frame;
use std::fmt::UpperHex;

pub const TOTAL_LINES: usize = 3;

enum LinesType {
    AC,
    PC,
    Halt,
}

fn map_index_to_line_type(index: usize) -> LinesType {
    match index {
        0 => LinesType::AC,
        1 => LinesType::PC,
        2 => LinesType::Halt,
        _ => panic!("Unknown status index line"),
    }
}

fn format_line<T>(name: &str, position: usize, uistate: &UIState, value: T) -> [String; 2]
where
    T: UpperHex
{
    let is_the_selected_line = match uistate.block_selected {
        BlockLists::Status => position == uistate.current_list().current_line,
        _ => false,
    };

    let line_name = format!("{}", name);
    let line_value = if is_the_selected_line && uistate.is_typing {
        format!("0x{}_", uistate.typing_char.unwrap())
    } else {
        format!("{:#04X}", value)
    };

    [line_name, line_value]
}

fn format_line_bool(name: &str, value: bool) -> [String; 2]
{
    let line_name = format!("{}", name);
    let line_value = format!("{}", value);
    [line_name, line_value]
}

pub fn draw<B>(uistate: &UIState, state: &State, f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let list_state = uistate.current_list();

    let lines: [[String; 2]; TOTAL_LINES] = [
        format_line("AC  ", 0, uistate, state.ac),
        format_line("PC  ", 1, uistate, state.pc),
        format_line_bool("HALT", state.halt),
    ];

    let list = lines
        .iter()
        .enumerate()
        .map(|(i, [name, value])|
            match uistate.block_selected {
                BlockLists::Status => if i == list_state.current_line {
                    Text::raw(format!(" -> {} {}", name, value))
                } else {
                    Text::raw(format!("    {} {}", name, value))
                }

                _ => Text::raw(format!("    {} {}", name, value))
            }
        );

    List::new(list.into_iter())
        .block(Block::default().borders(Borders::ALL).title(" Status "))
        .render(f, area);
}

// Actions
pub const STATUS_ACTIONS: ListActions = ListActions {
    move_up_cursor_handle: |list_state: &mut ListState| {
        if list_state.current_line == 0 {
            list_state.current_line = TOTAL_LINES - 1;
            return
        }

        list_state.current_line -= 1;
    },

    move_down_cursor_handle: |list_state: &mut ListState| {
        list_state.current_line = (list_state.current_line + 1) % TOTAL_LINES;
    },

    set_type_u8_handle: |state: &mut State, line_number: usize, u8_value: u8| {
        match map_index_to_line_type(line_number) {
            LinesType::AC => state.ac = u8_value,
            LinesType::PC => state.pc = u8_value as usize,
            LinesType::Halt => {},
        }
    },

    select_line_handle: |state: &mut State, line_number: usize| {
        match map_index_to_line_type(line_number) {
            LinesType::AC => state.ac = if state.ac == 0xFF { 0 } else { state.ac + 1 },
            LinesType::PC => state.pc = if state.pc == 0xFF { 0 } else { state.pc + 1 },
            LinesType::Halt => state.halt = !state.halt,
        }
    },
};
