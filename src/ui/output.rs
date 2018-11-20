use state::State;
use ui::tui::backend::Backend;
use ui::tui::widgets::{Widget, Block, Borders, Text, List};
use ui::tui::layout::Rect;
use ui::tui::Frame;

pub fn draw<B>(state: &State, f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let output_list = state.output
        .iter()
        .map(|value|
            Text::raw(format!(" {:#04X}", value))
        );

    List::new(output_list)
      .block(Block::default().borders(Borders::ALL).title(" Output "))
      .render(f, area);
}
