mod operator;
use state::operator::Operator;
use state::operator::get_operator;

pub struct State {
    memory: [u8; 255],
    pc: usize,
    ac: u8,
    inputs: [u8; 255],
    halt: bool,
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

    fn show(&self) {
        println!("PC: {}", self.pc);
        println!("AC: {}", self.ac);
        println!("--------");
    }

    pub fn start(self) -> State {
        let operator = self.fetch_operator();

        let operator_argument: u8;
        if operator.requires_arg {
            operator_argument = self.memory[self.pc + 1];
        } else {
            operator_argument = 0;
        }

        let new_state = self.execute_operator(operator, operator_argument);
        new_state.show();

        if new_state.halt == true {
            println!("Finish: halt");
            new_state
        } else if new_state.pc >= 255 {
            println!("Finish: end of memory");
            new_state
        } else {
            new_state.start()
        }
    }

    fn fetch_operator(&self) -> Operator {
        match get_operator(&self.memory[self.pc]) {
            Some(operator) => operator,
            None => panic!("Unknow OpCode: {:#04X}", self.memory[self.pc]),
        }
    }

    fn execute_operator(self, operator: Operator, operator_argument: u8) -> State {
        println!("{:?}", operator.mnemonic);
        (operator.run)(self, operator_argument)
    }

    pub fn memory_to_str(&self, limit: usize) -> std::vec::Vec<String> {
        let mut output: Vec<String> = vec![String::new(); 255];

        for (num, memory) in self.memory[..limit].iter().enumerate() {
            match get_operator(memory) {
                Some(operator) => output[num] = format!("[{:?}] {:#04X}", operator.mnemonic, memory).to_string(),
                None => output[num] = format!("[   ] {:#04X}", memory).to_string(),
            }
        }

        output
    }
}

