extern crate termion;
extern crate tui;
use state::State;
use std::io::{stdout, stdin};
use ui::tui::Terminal;
use ui::tui::backend::TermionBackend;
use ui::tui::widgets::{Widget, Block, Borders, Text, List};
use ui::tui::layout::{Layout, Constraint, Direction};
use ui::termion::{color, clear, cursor};
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
                .constraints([Constraint::Percentage(100), Constraint::Percentage(100)].as_ref())
                .split(size);

            let code_list_lines_size = (chunks[0].height - 2) as usize;
            let list_operators_result = final_state.list_operators(code_list_lines_size);
            let memory_str_table = list_operators_result
                .iter()
                .enumerate()
                .map(|(i, (operator, memory))| {
                     let memory_str = format!("{:#04X}", memory);
                     if i > 0 && list_operators_result[i - 1].0.requires_arg {
                         return Text::raw(
                            format!(
                                 " {:#04X}:           {red}{}{reset}",
                                 i, memory_str,
                                 red = color::Fg(color::Red),
                                 reset = color::Fg(color::Reset)
                            )
                        )
                     }

                     Text::raw(
                         format!(
                             " {:#04X}: {color}{:?}{blue} {goto}{}{reset}",
                             i, operator.mnemonic, memory_str,
                             color = color::Fg(color::Green),
                             blue = color::Fg(color::LightBlue),
                             goto = cursor::Left(18),
                             reset = color::Fg(color::Reset)
                         )
                     )
                });

            List::new(memory_str_table)
              .block(Block::default().borders(Borders::ALL).title(" Memory "))
              .render(&mut f, chunks[0]);
        })?;

        for c in stdin().keys() {
            match c? {
                Key::Char('q') => break 'main Ok(()),
                _ => {}
            }
        }
    }
}

