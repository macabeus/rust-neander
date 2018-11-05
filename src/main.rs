mod state;
use state::State;

fn main() {
    let state = State::new();
    state.start();
}

