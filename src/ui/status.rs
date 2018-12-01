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

fn format_line<T>(name: &str, value: T) -> [String; 2]
where
    T: UpperHex
{
    let line_name = format!("{}", name);
    let line_value = format!("{:#04X}", value);
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
        format_line("AC  ", state.ac),
        format_line("PC  ", state.pc),
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

    set_type_u8_handle: |_state: &mut State, _line_number: usize, _u8_value: u8| {
        // TODO
    },

    select_line_handle: |_state: &mut State, _line_number: usize| {
        // TODO
    },
};
