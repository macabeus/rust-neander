pub mod operator;
use state::operator::Operator;
use state::operator::NOP;
use state::operator::get_operator;

#[derive(Copy, Clone)]
pub struct State {
    memory: [u8; 255],
    pub pc: usize,
    pub ac: u8,
    inputs: [u8; 255],
    pub halt: bool,
}

impl State {
    pub fn new(memory: [u8; 255], inputs: [u8; 255]) -> State {
        State {
            memory,
            pc: 0,
            ac: 0,
            inputs,
            halt: false,
        }
    }

    pub fn next_tick(self) -> State {
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
        match get_operator(&self.memory[self.pc]) {
            Some(operator) => operator,
            None => panic!("Unknow OpCode: {:#04X}", self.memory[self.pc]),
        }
    }

    fn execute_operator(self, operator: Operator, operator_argument: u8) -> State {
        (operator.run)(self, operator_argument)
    }

    pub fn list_operators(&self) -> std::vec::Vec<(Operator, u8)> {
        let mut output: Vec<(Operator, u8)> = vec![(NOP, 0x00); 256];

        for (num, memory) in self.memory.iter().enumerate() {
            match get_operator(memory) {
                Some(operator) => output[num] = (operator, *memory),
                None => output[num] = (NOP, *memory),
            }
        }

        output
    }
}

