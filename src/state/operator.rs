use state::State;

#[derive(Copy, Clone, Debug)]
pub enum OpCode {
    NOP,
}

#[derive(Copy, Clone)]
pub struct Operator {
    pub mnemonic: OpCode,
    pub run: fn(State) -> State,
}

pub const NOP: Operator = Operator {
    mnemonic: OpCode::NOP,
    run: |state| {
        State {
            pc: state.pc + 1,
            ..state
        }
    },
};

pub fn get_operator(code: &u8) -> Option<Operator> {
    match code {
        0x00 ... 0x0F => Some(NOP),
        _ => None,
    }
}
