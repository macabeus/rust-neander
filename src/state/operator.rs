use state::State;

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    NOP,
    STA,
    LDA,
    ADD,
    OR,
    AND,
    NOT,
    SUB,
    JMP,
    JN,
    JZ,
    JNZ,
    IN,
    OUT,
    LDI,
    HLT,
}

#[derive(Copy, Clone)]
pub struct Operator {
    pub mnemonic: OpCode,
    pub requires_arg: bool,
    pub run: fn(State, u8) -> State,
}

pub const NOP: Operator = Operator {
    mnemonic: OpCode::NOP,
    requires_arg: false,
    run: |state, _| {
        State {
            pc: state.pc + 1,
            ..state
        }
    },
};

pub const STA: Operator = Operator {
    mnemonic: OpCode::STA,
    requires_arg: true,
    run: |state, argument| {
        let mut memory = state.memory;
        memory[argument as usize] = state.ac;

        State {
            pc: state.pc + 2,
            memory,
            ..state
        }
    }
};

pub const LDA: Operator = Operator {
    mnemonic: OpCode::LDA,
    requires_arg: true,
    run: |state, argument| {
        State {
            pc: state.pc + 2,
            ac: state.memory[argument as usize],
            ..state
        }
    }
};

pub const ADD: Operator = Operator {
    mnemonic: OpCode::ADD,
    requires_arg: true,
    run: |state, argument| {
        let memory_value = state.memory[argument as usize];

        State {
            pc: state.pc + 2,
            ac: state.ac + memory_value,
            ..state
        }
    }
};

pub const OR: Operator = Operator {
    mnemonic: OpCode::OR,
    requires_arg: true,
    run: |state, argument| {
        let memory_value = state.memory[argument as usize];

        State {
            pc: state.pc + 2,
            ac: memory_value | state.ac,
            ..state
        }
    }
};

pub const AND: Operator = Operator {
    mnemonic: OpCode::AND,
    requires_arg: true,
    run: |state, argument| {
        let memory_value = state.memory[argument as usize];

        State {
            pc: state.pc + 2,
            ac: memory_value & state.ac,
            ..state
        }
    }
};

pub const NOT: Operator = Operator {
    mnemonic: OpCode::NOT,
    requires_arg: false,
    run: |state, _| {
        State {
            pc: state.pc + 1,
            ac: !state.ac,
            ..state
        }
    }
};

pub const SUB: Operator = Operator {
    mnemonic: OpCode::SUB,
    requires_arg: true,
    run: |state, argument| {
        let memory_value = state.memory[argument as usize];

        State {
            pc: state.pc + 2,
            ac: state.ac - memory_value,
            ..state
        }
    }
};

pub const JMP: Operator = Operator {
    mnemonic: OpCode::JMP,
    requires_arg: true,
    run: |state, argument| {
        State {
            pc: argument as usize,
            ..state
        }
    }
};

pub const JN: Operator = Operator {
    mnemonic: OpCode::JN,
    requires_arg: true,
    run: |state, argument| {
        let next_pc = if state.ac >= 0b1000000 {
            argument as usize
        } else {
            state.pc + 2
        };

        State {
            pc: next_pc,
            ..state
        }
    }
};

pub const JZ: Operator = Operator {
    mnemonic: OpCode::JZ,
    requires_arg: true,
    run: |state, argument| {
        let next_pc = if state.ac == 0 {
            argument as usize
        } else {
            state.pc + 2
        };

        State {
            pc: next_pc,
            ..state
        }
    }
};

pub const JNZ: Operator = Operator {
    mnemonic: OpCode::JNZ,
    requires_arg: true,
    run: |state, argument| {
        let next_pc = if state.ac != 0 {
            argument as usize
        } else {
            state.pc + 2
        };

        State {
            pc: next_pc,
            ..state
        }
    }
};

pub const IN: Operator = Operator {
    mnemonic: OpCode::IN,
    requires_arg: true,
    run: |state, argument| {
        State {
            pc: state.pc + 2,
            ac: state.inputs[argument as usize],
            ..state
        }
    }
};

pub const OUT: Operator = Operator {
    mnemonic: OpCode::OUT,
    requires_arg: false,
    run: |state, _| {
        print!("{}", state.ac);

        State {
            pc: state.pc + 1,
            ..state
        }
    }
};

pub const LDI: Operator = Operator {
    mnemonic: OpCode::LDI,
    requires_arg: true,
    run: |state, argument| {
        State {
            pc: state.pc + 2,
            ac: argument,
            ..state
        }
    }
};

pub const HLT: Operator = Operator {
    mnemonic: OpCode::HLT,
    requires_arg: false,
    run: |state, _| {
        State {
            pc: state.pc + 1,
            halt: true,
            ..state
        }
    },
};

pub fn get_operator(code: &u8) -> Option<Operator> {
    match code {
        0x00 ... 0x0F => Some(NOP),
        0x10 ... 0x1F => Some(STA),
        0x20 ... 0x2F => Some(LDA),
        0x30 ... 0x3F => Some(ADD),
        0x40 ... 0x4F => Some(OR),
        0x50 ... 0x5F => Some(AND),
        0x60 ... 0x6F => Some(NOT),
        0x70 ... 0x7F => Some(SUB),
        0x80 ... 0x8F => Some(JMP),
        0x90 ... 0x9F => Some(JN),
        0xA0 ... 0xAF => Some(JZ),
        0xB0 ... 0xBF => Some(JNZ),
        0xC0 ... 0xCF => Some(IN),
        0xD0 ... 0xDF => Some(OUT),
        0xE0 ... 0xEF => Some(LDI),
        0xF0 ... 0xFF => Some(HLT),
        _ => None,
    }
}
