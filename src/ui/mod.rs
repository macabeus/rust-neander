extern crate termion;
extern crate tui;
mod memory_list;
mod status;
mod uistate;
mod action;
use ui::uistate::UIState;
use state::State;
use std::io::stdout;
use ui::tui::Terminal;
use ui::tui::backend::TermionBackend;
use ui::tui::layout::{Layout, Constraint, Direction};
use ui::termion::clear;
use ui::termion::raw::IntoRawMode;

pub fn draw_screen(state: State) -> Result<(), Box<std::error::Error>> {
    let stdout_raw_mode = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout_raw_mode);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let size = terminal.size()?;

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(100)].as_ref())
        .split(size);
    let memory_list_count_line = (chunks[1].height - 3) as usize;

    let mut uistate = UIState {
        current_line: 0,
        memory_list_first_line: 0,
        memory_list_last_line: memory_list_count_line,
        is_typing: false,
        typing_char: None,
        quit: false,
    };

    let mut current_state = state;

    println!("{}", clear::All);

    loop {
        terminal.draw(|mut f| {
            status::draw(&current_state, &mut f, chunks[0]);
            memory_list::draw(&uistate, &current_state, &mut f, chunks[1]);
        })?;

        let input = action::wait_for_valid_input();
        action::execute(input, &mut current_state, &mut uistate);

        if uistate.quit {
            break Ok(())
        }
    }
}

