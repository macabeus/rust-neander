extern crate termion;
extern crate tui;
mod memory_list;
mod status;
mod uistate;
use ui::uistate::UIState;
use state::State;
use std::io::{stdout, stdin};
use ui::tui::Terminal;
use ui::tui::backend::TermionBackend;
use ui::tui::layout::{Layout, Constraint, Direction};
use ui::termion::clear;
use ui::termion::event::Key;
use ui::termion::input::TermRead;
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
    };

    let mut current_state = state;

    println!("{}", clear::All);

    'main: loop {
        terminal.draw(|mut f| {
            status::draw(&current_state, &mut f, chunks[0]);
            memory_list::draw(&uistate, &current_state, &mut f, chunks[1]);
        })?;

        for c in stdin().keys() {
            match c? {
                Key::Char('q') => break 'main Ok(()),
                Key::Char('n') => {
                    current_state = current_state.next_tick();
                    break;
                },
                Key::Up => {
                    if uistate.current_line == 0 {
                        break;
                    }

                    uistate.current_line -= 1;

                    if uistate.memory_list_first_line + 3 > uistate.current_line && uistate.memory_list_first_line > 0 {
                        uistate.memory_list_first_line -= 1;
                        uistate.memory_list_last_line -= 1;
                    }

                    break;
                },
                Key::Down => {
                    if uistate.current_line == 0xFF {
                        break;
                    }

                    uistate.current_line += 1;

                    if uistate.memory_list_last_line - 3 < uistate.current_line && uistate.memory_list_last_line < 0xFF {
                        uistate.memory_list_first_line += 1;
                        uistate.memory_list_last_line += 1;
                    }

                    break;
                },
                _ => {}
            }
        }
    }
}

