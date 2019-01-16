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
