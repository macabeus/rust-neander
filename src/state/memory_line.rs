use state::operator::Operator;
use state::operator::NOP;

#[derive(Copy, Clone)]
pub enum LineKind {
    Operator,
    Argument,
}

#[derive(Copy, Clone)]
pub struct MemoryLine {
    pub operator: Operator,
    pub value: u8,
    pub kind: LineKind,
}

pub const MEMORY_LINE_BLANK: MemoryLine = MemoryLine {
    operator: NOP,
    value: 0x00,
    kind: LineKind::Operator,
};
