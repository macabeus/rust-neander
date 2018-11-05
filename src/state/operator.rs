use state::State;

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    NOP,
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
        0xF0 ... 0xFF => Some(HLT),
        _ => None,
    }
}
