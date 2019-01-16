use super::super::state::State;
use super::operator;

fn compare_u8_slices(slice1: &[u8], slice2: &[u8]) {
    for (i, item) in slice1.iter().enumerate() {
        assert_eq!(item, &slice2[i]);
    }
}

#[test]
fn nop() {
    let state = State::new([0; 255], [0; 255]);

    let new_state = (operator::NOP.run)(&state, 0);

    assert_eq!(new_state.pc, state.pc + 1);

    assert_eq!(new_state.ac, state.ac);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn sta() {
    let mut state = State::new([0; 255], [0; 255]);

    state.ac = 25;

    let new_state = (operator::STA.run)(&state, 50);

    assert_eq!(new_state.pc, state.pc + 2);

    assert_eq!(new_state.ac, state.ac);

    assert_eq!(new_state.halt, false);

    for (i, item) in new_state.memory.iter().enumerate() {
        if i == 50 {
            assert_eq!(item, &state.ac);
        } else {
            assert_eq!(item, &state.memory[i]);
        }
    }

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn lda() {
    let mut state = State::new([0; 255], [0; 255]);

    state.memory[50] = 25;

    let new_state = (operator::LDA.run)(&state, 50);

    assert_eq!(new_state.pc, state.pc + 2);

    assert_eq!(new_state.ac, state.memory[50]);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn add() {
    let mut state = State::new([0; 255], [0; 255]);

    state.memory[50] = 20;

    state.ac = 10;

    let new_state = (operator::ADD.run)(&state, 50);

    assert_eq!(new_state.pc, state.pc + 2);

    assert_eq!(new_state.ac, state.ac + 20);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn or() {
    let mut state = State::new([0; 255], [0; 255]);

    state.memory[50] = 20;

    state.ac = 10;

    let new_state = (operator::OR.run)(&state, 50);

    assert_eq!(new_state.pc, state.pc + 2);

    assert_eq!(new_state.ac, state.ac | 20);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn and() {
    let mut state = State::new([0; 255], [0; 255]);

    state.memory[50] = 20;

    state.ac = 10;

    let new_state = (operator::AND.run)(&state, 50);

    assert_eq!(new_state.pc, state.pc + 2);

    assert_eq!(new_state.ac, state.ac & 20);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn not() {
    let mut state = State::new([0; 255], [0; 255]);

    state.ac = 10;

    let new_state = (operator::NOT.run)(&state, 0);

    assert_eq!(new_state.pc, state.pc + 1);

    assert_eq!(new_state.ac, !state.ac);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn sub() {
    let mut state = State::new([0; 255], [0; 255]);

    state.memory[50] = 5;

    state.ac = 10;

    let new_state = (operator::SUB.run)(&state, 50);

    assert_eq!(new_state.pc, state.pc + 2);

    assert_eq!(new_state.ac, state.ac - 5);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn jmp() {
    let state = State::new([0; 255], [0; 255]);

    let new_state = (operator::JMP.run)(&state, 50);

    assert_eq!(new_state.pc, 50);

    assert_eq!(new_state.ac, state.ac);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn jn_when_accumulator_is_negative() {
    let mut state = State::new([0; 255], [0; 255]);

    state.ac = 0b1000000;

    let new_state = (operator::JN.run)(&state, 50);

    assert_eq!(new_state.pc, 50);

    assert_eq!(new_state.ac, state.ac);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn jn_when_accumulator_is_positive() {
    let mut state = State::new([0; 255], [0; 255]);

    state.ac = 0;

    let new_state = (operator::JN.run)(&state, 50);

    assert_eq!(new_state.pc, state.pc + 2);

    assert_eq!(new_state.ac, state.ac);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn jz_when_accumulator_is_zero() {
    let mut state = State::new([0; 255], [0; 255]);

    state.ac = 0;

    let new_state = (operator::JZ.run)(&state, 50);

    assert_eq!(new_state.pc, 50);

    assert_eq!(new_state.ac, state.ac);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}

#[test]
fn jz_when_accumulator_is_not_zero() {
    let mut state = State::new([0; 255], [0; 255]);

    state.ac = 2;

    let new_state = (operator::JZ.run)(&state, 50);

    assert_eq!(new_state.pc, state.pc + 2);

    assert_eq!(new_state.ac, state.ac);

    assert_eq!(new_state.halt, false);

    compare_u8_slices(&new_state.memory, &state.memory);

    compare_u8_slices(&new_state.output, &state.output);
}
