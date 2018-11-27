use ui::uistate::ListState;
use state::State;

pub struct ListActions {
    pub move_up_cursor_handle: fn(&mut ListState),
    pub move_down_cursor_handle: fn(&mut ListState),
    pub set_type_u8_handle: fn(&mut State, usize, u8),
    pub select_line_handle: fn(&mut State, usize),
}
