use ui::tui::layout::Rect;
use ui::tui::backend::Backend;
use ui::tui::widgets::{Widget, Block, Borders, Paragraph, Text};
use ui::tui::Frame;

pub fn draw<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let texts = [
        Text::raw("[00] NOP | "),
        Text::raw("[10] STA address | "),
        Text::raw("[20] LDA address | "),
        Text::raw("[30] ADD address | "),
        Text::raw("[40] OR address | "),
        Text::raw("[50] AND address | "),
        Text::raw("[60] NOT | "),
        Text::raw("[70] SUB address | "),
        Text::raw("[80] JMP value | "),
        Text::raw("[90] JN value | "),
        Text::raw("[A0] JZ value | "),
        Text::raw("[B0] JNZ value | "),
        Text::raw("[C0] IN index | "),
        Text::raw("[D0] OUT | "),
        Text::raw("[E0] LDI value | "),
        Text::raw("[F0] HLT"),
    ];

    Paragraph::new(texts.into_iter())
        .block(Block::default().borders(Borders::ALL).title(" Opcodes "))
        .wrap(true)
        .render(f, area);
}
