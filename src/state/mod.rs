pub mod operator;
pub mod memory_line;
use state::operator::Operator;
use state::operator::get_operator;
use state::memory_line::LineKind;
use state::memory_line::MemoryLine;
use state::memory_line::MEMORY_LINE_BLANK;

pub struct State {
    pub memory: [u8; 255],
    pub pc: usize,
    pub ac: u8,
    inputs: [u8; 255],
    pub halt: bool,
    pub output: [u8; 40],
}

impl State {
    pub fn new(memory: [u8; 255], inputs: [u8; 255]) -> State {
        State {
            memory,
            pc: 0,
            ac: 0,
            inputs,
            halt: false,
            output: [0x00; 40],
        }
    }

    pub fn next_tick(&self) -> State {
        let operator = self.fetch_operator();

        let operator_argument: u8;
        if operator.requires_arg {
            operator_argument = self.memory[self.pc + 1];
        } else {
            operator_argument = 0;
        }

        let mut new_state = self.execute_operator(operator, operator_argument);

        if new_state.pc >= 255 {
            new_state.halt = true;
        }

        new_state
    }

    fn fetch_operator(&self) -> Operator {
        get_operator(&self.memory[self.pc])
    }

    fn execute_operator(&self, operator: Operator, operator_argument: u8) -> State {
        (operator.run)(self, operator_argument)
    }

    pub fn list_operators(&self) -> std::vec::Vec<MemoryLine> {
        let mut output: Vec<MemoryLine> = vec![MEMORY_LINE_BLANK; 256];
        let mut line_kind = LineKind::Operator;

        for (num, memory) in self.memory.iter().enumerate() {
            let operator = get_operator(memory);
            output[num] = MemoryLine {
                operator,
                value: *memory,
                kind: line_kind,
            };

            match line_kind {
                LineKind::Operator => {
                    if operator.requires_arg {
                        line_kind = LineKind::Argument;
                    }
                },
                LineKind::Argument => {
                    line_kind = LineKind::Operator;
                },
            }
        }

        output
    }
}

