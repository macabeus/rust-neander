extern crate termion;
extern crate tui;
mod memory_list;
mod status;
mod uistate;
mod action;
mod output;
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

    let chunks_main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(size);
    let chunks_left_bar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks_main[0]);
    let status_chunk = chunks_left_bar[0];
    let output_chunk = chunks_left_bar[1];
    let memory_list_chunk = chunks_main[1];

    let memory_list_count_line = (memory_list_chunk.height - 3) as usize;

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
            status::draw(&current_state, &mut f, status_chunk);
            output::draw(&current_state, &mut f, output_chunk);
            memory_list::draw(&uistate, &current_state, &mut f, memory_list_chunk);
        })?;

        let input = action::wait_for_valid_input();
        action::execute(input, &mut current_state, &mut uistate);

        if uistate.quit {
            break Ok(())
        }
    }
}

