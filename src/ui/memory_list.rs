use state::operator::Operator;
use ui::termion::{color, cursor};

macro_rules! MNEMONIC_COLOR { () => { color::Fg(color::Green) } }
macro_rules! OPERATOR_MEMORY_COLOR { () => { color::Fg(color::LightBlue) } }
macro_rules! ARGUMENT_COLOR { () => { color::Fg(color::Red) } }

fn format_operator_line(position: &usize, operator: &Operator, operator_memory: &u8) -> String {
    format!(
        " {:#04X}: {mnemonic_color}{:?}{operator_memory_color} {goto}{:#04X}{reset}",
        position, operator.mnemonic, operator_memory,
        mnemonic_color = MNEMONIC_COLOR!(),
        operator_memory_color = OPERATOR_MEMORY_COLOR!(),
        goto = cursor::Left(18),
        reset = color::Fg(color::Reset)
    )
}

fn format_argument_line(position: &usize, argument: &u8) -> String {
    format!(
        " {:#04X}:           {argument_color}{:#04X}{reset}",
        position, argument,
        argument_color = ARGUMENT_COLOR!(),
        reset = color::Fg(color::Reset)
    )
}

fn previous_argument_require_argument(memory: &std::vec::Vec<(Operator, u8)>, position: &usize) -> bool {
    if position == &0 {
        return false
    }

    let previous_operator = &memory[position - 1].0;
    previous_operator.requires_arg
}

pub fn format_memory_line(
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

