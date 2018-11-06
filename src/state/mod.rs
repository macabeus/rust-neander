mod operator;
use state::operator::Operator;
use state::operator::get_operator;

pub struct State {
    memory: [u8; 255],
    pc: usize,
    ac: u8,
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
            ac: 0,
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

    pub fn print_memory(self, limit: usize) {
        for i in self.memory[..limit].iter() {
            match get_operator(i) {
                Some(operator) => println!("[{:?}] {:#04X}", operator.mnemonic, i),
                None => println!("[   ] {:#04X}", i),
            }
        }
    }
}

