mod state;
use state::State;

fn main() {
    let state = State::new(&[
        0x00,
        0x20,
        0x0A,
        0x10,
        0x0F,
        0x30,
        0x0F,
        0x40,
        0x00,
        0x50,
        0x0F,
        0x60,
        0x70,
        0x0F,
        0xFF,
        0x05
    ]);
    let final_state = state.start();

    println!("--- FINAL MEMORY ---");
    final_state.print_memory(20);
}

