mod state;
use state::State;

fn main() {
    let state = State::new(&[0x00, 0x20, 0x06, 0x10, 0x07, 0xFF, 0x05]);
    let final_state = state.start();

    println!("--- FINAL MEMORY ---");
    final_state.print_memory(10);
}

