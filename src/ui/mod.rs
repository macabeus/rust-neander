extern crate termion;
extern crate tui;
mod memory_list;
mod status;
use state::State;
use std::io::{stdout, stdin};
use ui::tui::Terminal;
use ui::tui::backend::TermionBackend;
use ui::tui::widgets::{Widget, Block, Borders, Text, List};
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
            // Layout
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(100)].as_ref())
                .split(size);

            // Status
            status::draw(&final_state, &mut f, chunks[0]);

            // Memory list
            let code_list_lines_size = (chunks[1].height - 2) as usize;
            let list_operators_result = final_state.list_operators(code_list_lines_size);
            let memory_str_table = list_operators_result
                .iter()
                .enumerate()
                .map(|(i, (operator, memory))|
                    Text::raw(
                        memory_list::format_memory_line(
                            &list_operators_result,
                            &i,
                            &operator,
                            &memory
                        )
                    )
                );

            List::new(memory_str_table)
              .block(Block::default().borders(Borders::ALL).title(" Memory "))
              .render(&mut f, chunks[1]);
        })?;

        for c in stdin().keys() {
            match c? {
                Key::Char('q') => break 'main Ok(()),
                _ => {}
            }
        }
    }
}

