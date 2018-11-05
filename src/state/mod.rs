mod operator;
use state::operator::Operator;
use state::operator::get_operator;

pub struct State {
    memory: [u8; 255],
    pc: usize,
    halt: bool,
}

impl State {
    pub fn new(memory: &[u8]) -> State {
        let mut new_memory = [0x00; 255];
        for (i, item) in memory.iter().enumerate() {
            new_memory[i] = *item;
        }

        State {
            memory: new_memory,
            pc: 0,
            halt: false,
        }
    }

    fn show(&self) {
        println!("PC: {}", self.pc);
        println!("--------");
    }

    pub fn start(self) {
        let operator = self.fetch_operator();

        let new_state = self.execute_operator(operator);
        new_state.show();

        if new_state.halt == true {
            println!("Finish: halt")
        } else if new_state.pc >= 255 {
            println!("Finish: end of memory")
        } else {
            new_state.start();
        }
    }

    fn fetch_operator(&self) -> Operator {
        match get_operator(&self.memory[self.pc]) {
            Some(operator) => operator,
            None => panic!("Unknow OpCode: {:#04X}", self.memory[self.pc]),
        }
    }

    fn execute_operator(self, operator: Operator) -> State {
        println!("{:?}", operator.mnemonic);
        (operator.run)(self)
    }
}

