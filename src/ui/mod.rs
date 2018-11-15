extern crate termion;
extern crate tui;
mod memory_list;
mod status;
use state::State;
use std::io::{stdout, stdin};
use ui::tui::Terminal;
use ui::tui::backend::TermionBackend;
use ui::tui::layout::{Layout, Constraint, Direction};
use ui::termion::clear;
use ui::termion::event::Key;
use ui::termion::input::TermRead;
use ui::termion::raw::IntoRawMode;

pub fn draw_screen(final_state: State) -> Result<(), Box<std::error::Error>> {
    let stdout_raw_mode = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout_raw_mode);
    let mut terminal = Terminal::new(backend)?;

    let size = terminal.size()?;

    println!("{}", clear::All);

    'main: loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(100)].as_ref())
                .split(size);

            status::draw(&final_state, &mut f, chunks[0]);
            memory_list::draw(&final_state, &mut f, chunks[1]);
        })?;

        for c in stdin().keys() {
            match c? {
                Key::Char('q') => break 'main Ok(()),
                _ => {}
            }
        }
    }
}

