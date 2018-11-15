use state::State;
use ui::tui::backend::Backend;
use ui::tui::widgets::{Widget, Block, Borders, List, Text};
use ui::tui::layout::Rect;
use ui::tui::Frame;
use std::fmt::UpperHex;

fn format_line<T>(name: &str, value: T) -> [String; 2]
where
    T: UpperHex
{
    let line_name = format!(" {}", name);
    let line_value = format!("{:#04X}", value);
    [line_name, line_value]
}

fn format_line_bool(name: &str, value: bool) -> [String; 2]
{
    let line_name = format!(" {}", name);
    let line_value = format!("{}", value);
    [line_name, line_value]
}

pub fn draw<B>(state: &State, f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let lines = [
        format_line("AC  ", state.ac),
        format_line("PC  ", state.pc),
        format_line_bool("HALT", state.halt)
    ];

    let list = lines
        .iter()
        .map(|[name, value]|
             Text::raw(format!("{} {}", name, value))
        );

    List::new(list.into_iter())
        .block(Block::default().borders(Borders::ALL).title(" Status "))
        .render(f, area);
}
