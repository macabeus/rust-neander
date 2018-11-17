use state::State;
use state::operator::Operator;
use ui::uistate::UIState;
use ui::tui::backend::Backend;
use ui::tui::widgets::{Widget, Block, Borders, Text, List};
use ui::tui::layout::Rect;
use ui::tui::Frame;

fn format_operator_line(position: &usize, operator: &Operator, operator_memory: &u8) -> String {
    format!(
        "{:#04X}: {:?}  {:#04X}",
        position, operator.mnemonic, operator_memory,
    )
}

fn format_argument_line(position: &usize, argument: &u8) -> String {
    format!(
        "{:#04X}:           {:#04X}",
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
    uistate: &UIState,
    memory: &std::vec::Vec<(Operator, u8)>,
    position: &usize,
    operator: &Operator,
    memory_value: &u8
) -> String {
    let line = if previous_argument_require_argument(memory, position) {
        format_argument_line(position, memory_value)
    } else {
        format_operator_line(position, operator, memory_value)
    };

    if *position == uistate.current_line {
        format!(" -> {}", line)
    } else {
        format!("    {}", line)
    }
}

pub fn draw<B>(uistate: &UIState, current_state: &State, f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let list_operators = current_state.list_operators();
    let list_operators_slice = &list_operators[uistate.memory_list_first_line..=uistate.memory_list_last_line];

    let memory_str_table = list_operators_slice
        .iter()
        .enumerate()
        .map(|(i, (operator, memory))|
            Text::raw(
                format_memory_line(
                    uistate,
                    &list_operators,
                    &(i + uistate.memory_list_first_line),
                    &operator,
                    &memory
                )
            )
        );

    List::new(memory_str_table)
      .block(Block::default().borders(Borders::ALL).title(" Memory "))
      .render(f, area);
}
