mod state;
use state::State;

fn main() {
    let state = State::new(&[
        0x00,
        0x20,
        0x0A,
        0x10,
        0x0B,
        0x30,
        0x0B,
        0x40,
        0x00,
        0xFF,
        0x05
    ]);
    let final_state = state.start();

    println!("--- FINAL MEMORY ---");
    final_state.print_memory(20);
}

