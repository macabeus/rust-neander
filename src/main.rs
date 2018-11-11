#[macro_use]
extern crate clap;
mod state;
mod io;
mod ui;
use state::State;
use clap::App;

struct CLICommands {
    filename: String,
    input: String,
}

fn extract_cli_commands() -> CLICommands {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    CLICommands {
        filename: matches.value_of("FILEPATH").unwrap().to_string(),
        input: matches.value_of("input").unwrap_or("").to_string(),
    }
}

fn main() -> Result<(), Box<std::error::Error>> {
    let cli_commands = extract_cli_commands();
    let memory = io::load_file(&cli_commands.filename);
    let inputs = io::load_inputs(&cli_commands.input);

    let state = State::new(memory, inputs);
    let final_state = state.start();

    ui::draw_screen(final_state)
}

