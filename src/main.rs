mod state;
use state::State;

fn main() {
    let state = State::new(&[0x00, 0x20, 0x08, 0x10, 0x09, 0x30, 0x09, 0xFF, 0x05]);
    let final_state = state.start();

    println!("--- FINAL MEMORY ---");
    final_state.print_memory(10);
}

