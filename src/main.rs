mod state;
use state::State;

fn main() {
    let state = State::new(&[0x00, 0x20, 0x04, 0xFF, 0x05]);
    state.start();
}

