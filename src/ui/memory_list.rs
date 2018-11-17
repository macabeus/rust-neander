use state::State;
use state::operator::Operator;
use ui::tui::backend::Backend;
use ui::tui::widgets::{Widget, Block, Borders, Text, List};
use ui::tui::layout::Rect;
use ui::tui::Frame;

fn format_operator_line(position: &usize, operator: &Operator, operator_memory: &u8) -> String {
    format!(
        " {:#04X}: {:?}  {:#04X}",
        position, operator.mnemonic, operator_memory,
    )
}

fn format_argument_line(position: &usize, argument: &u8) -> String {
    format!(
        " {:#04X}:           {:#04X}",
        position, argument,
    )
}

fn previous_argument_require_argument(memory: &std::vec::Vec<(Operator, u8)>, position: &usize) -> bool {
    if position == &0 {
        return false
    }

    let previous_operator = &memory[position - 1].0;
    previous_operator.requires_arg
}

fn format_memory_line(
    memory: &std::vec::Vec<(Operator, u8)>,
    position: &usize,
    operator: &Operator,
    memory_value: &u8
) -> String {
    if previous_argument_require_argument(memory, position) {
        format_argument_line(position, memory_value)
    } else {
        format_operator_line(position, operator, memory_value)
    }
}

pub fn draw<B>(final_state: &State, f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let code_list_lines_size = (area.height - 2) as usize;
    let list_operators_result = final_state.list_operators(code_list_lines_size);
    let memory_str_table = list_operators_result
        .iter()
        .enumerate()
        .map(|(i, (operator, memory))|
            Text::raw(
                format_memory_line(
                    &list_operators_result,
                    &i,
                    &operator,
                    &memory
                )
            )
        );

    List::new(memory_str_table)
      .block(Block::default().borders(Borders::ALL).title(" Memory "))
      .render(f, area);
}
