mod state;
use state::State;

fn main() {
    let state = State::new(&[0x00, 0x00]);
    state.start();
}

