use io;
use state::State;
use ui::uistate::BlockLists;
use ui::uistate::UIState;
use std::io::stdin;
use ui::termion::input::TermRead;
use ui::termion::event::Key;

pub enum Input {
    NextTick,
    Play,
    ToggleHalt,
    MoveUpCursor,
    MoveDownCursor,
    ChangeBlockSelected,
    TypeChar(char),
    CancelType,
    SetPC,
    Save,
    Quit,
}

pub fn wait_for_valid_input() -> Input {
    loop { for c in stdin().keys() {
        match c.unwrap() {
            Key::Char('n') => return Input::NextTick,
            Key::Char('p') => return Input::Play,
            Key::Char('h') => return Input::ToggleHalt,
            Key::Up => return Input::MoveUpCursor,
            Key::Down => return Input::MoveDownCursor,
            Key::Char('\t') => return Input::ChangeBlockSelected,
            Key::Esc => return Input::CancelType,
            Key::Char(' ') => return Input::SetPC,
            Key::Char('s') => return Input::Save,
            Key::Char('q') => return Input::Quit,
            Key::Char(key) => {
                match key {
                    c @ '0'...'9' | c @ 'A'...'F' | c @ 'a'...'f' => return Input::TypeChar(c),
                    _ => {}
                }
            },
            _ => {},
        };
    } }
}

pub fn execute(input: Input, state: &mut State, uistate: &mut UIState) {
    match input {
        Input::NextTick => next_tick_handle(state),
        Input::Play => play_handle(state),
        Input::ToggleHalt => toggle_halt_handle(state),
        Input::MoveUpCursor => move_up_cursor_handle(uistate),
        Input::MoveDownCursor => move_down_cursor_handle(uistate),
        Input::ChangeBlockSelected => change_block_selected_handle(uistate),
        Input::TypeChar(key) => type_char_handle(key, state, uistate),
        Input::CancelType => cancel_type_handle(uistate),
        Input::SetPC => set_pc_handle(state, uistate),
        Input::Save => save_handle(state, uistate),
        Input::Quit => quit_handle(uistate),
    }
}

fn next_tick_handle(state: &mut State) {
    *state = state.next_tick();
}

fn play_handle(state: &mut State) {
    state.play();
}

fn toggle_halt_handle(state: &mut State) {
    state.halt = !state.halt;
}

fn move_up_cursor_handle(uistate: &mut UIState) {
    (uistate.current_list().handle_action.move_up_cursor_handle)(uistate.mutable_current_list());
}

fn move_down_cursor_handle(uistate: &mut UIState) {
    (uistate.current_list().handle_action.move_down_cursor_handle)(uistate.mutable_current_list());
}

fn change_block_selected_handle(uistate: &mut UIState) {
    match uistate.block_selected {
        BlockLists::Status => uistate.block_selected = BlockLists::Operators,
        BlockLists::Operators => uistate.block_selected = BlockLists::Variables,
        BlockLists::Variables => uistate.block_selected = BlockLists::Status,
    }
}

fn type_char_handle(key: char, state: &mut State, uistate: &mut UIState) {
    if uistate.is_typing {
        let s = format!("{}{}", uistate.typing_char.unwrap(), key).to_string();
        let u8_typed = u8::from_str_radix(&s, 16).unwrap();
        (uistate.current_list().handle_action.set_type_u8_handle)(state, uistate.current_list().current_line, u8_typed);

        uistate.is_typing = false;
        uistate.typing_char = None;
    } else {
        uistate.is_typing = true;
        uistate.typing_char = Some(key);
    }
}

fn cancel_type_handle(uistate: &mut UIState) {
    uistate.is_typing = false;
    uistate.typing_char = None;
}

fn set_pc_handle(state: &mut State, uistate: &mut UIState) {
    (uistate.current_list().handle_action.select_line_handle)(state, uistate.current_list().current_line);
}

fn save_handle(state: &mut State, uistate: &mut UIState) {
    io::save_memory(&uistate.filepath, state.memory);
}

fn quit_handle(uistate: &mut UIState) {
    uistate.quit = true;
}

