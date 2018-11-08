mod state;
use state::State;

fn main() {
    let state = State::new(&[
        0x00,
        0x20,
        0x0A,
        0x10,
        0x11,
        0x30,
        0x11,
        0x40,
        0x00,
        0x50,
        0x11,
        0x60,
        0x70,
        0x11,
        0x80,
        0x14,
        0xFF,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0xFF,
    ]);
    let final_state = state.start();

    println!("--- FINAL MEMORY ---");
    final_state.print_memory(30);
}

