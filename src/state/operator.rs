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
    pub run: fn(&mut State, u8),
}

pub const NOP: Operator = Operator {
    mnemonic: OpCode::NOP,
    requires_arg: false,
    run: |state, _| {
        state.pc += 1;
    },
};

pub const STA: Operator = Operator {
    mnemonic: OpCode::STA,
    requires_arg: true,
    run: |state, argument| {
        let mut memory = state.memory;
        memory[argument as usize] = state.ac;

        state.pc += 2;
        state.memory = memory;
    }
};

pub const LDA: Operator = Operator {
    mnemonic: OpCode::LDA,
    requires_arg: true,
    run: |state, argument| {
        state.pc += 2;
        state.ac = state.memory[argument as usize];
    }
};

pub const ADD: Operator = Operator {
    mnemonic: OpCode::ADD,
    requires_arg: true,
    run: |state, argument| {
        let memory_value = state.memory[argument as usize];

        state.pc += 2;
        state.ac += memory_value;
    }
};

pub const OR: Operator = Operator {
    mnemonic: OpCode::OR,
    requires_arg: true,
    run: |state, argument| {
        let memory_value = state.memory[argument as usize];

        state.pc += 2;
        state.ac = memory_value | state.ac;
    }
};

pub const AND: Operator = Operator {
    mnemonic: OpCode::AND,
    requires_arg: true,
    run: |state, argument| {
        let memory_value = state.memory[argument as usize];

        state.pc += 2;
        state.ac = memory_value & state.ac;
    }
};

pub const NOT: Operator = Operator {
    mnemonic: OpCode::NOT,
    requires_arg: false,
    run: |state, _| {
        state.pc += 1;
        state.ac = !state.ac;
    }
};

pub const SUB: Operator = Operator {
    mnemonic: OpCode::SUB,
    requires_arg: true,
    run: |state, argument| {
        let memory_value = state.memory[argument as usize];

        state.pc += 2;
        state.ac -= memory_value;
    }
};

pub const JMP: Operator = Operator {
    mnemonic: OpCode::JMP,
    requires_arg: true,
    run: |state, argument| {
        state.pc = argument as usize;
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

        state.pc = next_pc;
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

        state.pc = next_pc;
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

        state.pc = next_pc;
    }
};

pub const IN: Operator = Operator {
    mnemonic: OpCode::IN,
    requires_arg: true,
    run: |state, argument| {
        state.pc += 2;
        state.ac = state.inputs[argument as usize];
    }
};

pub const OUT: Operator = Operator {
    mnemonic: OpCode::OUT,
    requires_arg: false,
    run: |state, _| {
        let mut output = [0x00; 40];
        output[0] = state.ac;
        for (i, value) in state.output[..39].iter().enumerate() {
            output[i + 1] = *value;
        }

        state.pc += 1;
        state.output = output;
    }
};

pub const LDI: Operator = Operator {
    mnemonic: OpCode::LDI,
    requires_arg: true,
    run: |state, argument| {
        state.pc += 2;
        state.ac = argument;
    }
};

pub const HLT: Operator = Operator {
    mnemonic: OpCode::HLT,
    requires_arg: false,
    run: |state, _| {
        state.pc += 1;
        state.halt = true;
    },
};

pub fn get_operator(code: &u8) -> Operator {
    match code {
        0x00 ... 0x0F => NOP,
        0x10 ... 0x1F => STA,
        0x20 ... 0x2F => LDA,
        0x30 ... 0x3F => ADD,
        0x40 ... 0x4F => OR,
        0x50 ... 0x5F => AND,
        0x60 ... 0x6F => NOT,
        0x70 ... 0x7F => SUB,
        0x80 ... 0x8F => JMP,
        0x90 ... 0x9F => JN,
        0xA0 ... 0xAF => JZ,
        0xB0 ... 0xBF => JNZ,
        0xC0 ... 0xCF => IN,
        0xD0 ... 0xDF => OUT,
        0xE0 ... 0xEF => LDI,
        0xF0 ... 0xFF => HLT,
        opcode => panic!("Unknow OpCode: {:#04X}", opcode),
    }
}
