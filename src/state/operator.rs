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
        0xF0 ... 0xFF => Some(HLT),
        _ => None,
    }
}
