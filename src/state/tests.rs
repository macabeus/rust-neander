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
