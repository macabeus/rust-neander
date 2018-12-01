extern crate termion;
extern crate tui;
mod memory_list;
mod status;
mod uistate;
mod action;
mod output;
mod list_action;
mod references;
use ui::uistate::BlockLists;
use ui::uistate::ListState;
use ui::uistate::UIState;
use state::State;
use std::io::stdout;
use ui::tui::Terminal;
use ui::tui::backend::TermionBackend;
use ui::tui::layout::{Layout, Constraint, Direction};
use ui::termion::clear;
use ui::termion::raw::IntoRawMode;

pub fn draw_screen(state: State) -> Result<(), Box<std::error::Error>> {
    let stdout_raw_mode = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout_raw_mode);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let size = terminal.size()?;

    let chunks_main_and_references = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(4)].as_ref())
        .split(size);
    let chunks_main = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(40), Constraint::Percentage(40)].as_ref())
        .split(chunks_main_and_references[0]);
    let chunks_left_bar = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(chunks_main[0]);
    let references_chunk = chunks_main_and_references[1];
    let status_chunk = chunks_left_bar[0];
    let output_chunk = chunks_left_bar[1];
    let memory_list_operators_chunk = chunks_main[1];
    let memory_list_variables_chunk = chunks_main[2];

    let memory_list_operators_count_line = (memory_list_operators_chunk.height - 3) as usize;
    let memory_list_variables_count_line = (memory_list_variables_chunk.height - 3) as usize;

    let mut uistate = UIState {
        block_selected: BlockLists::Operators,
        status_block: ListState {
            current_line: 0,
            first_line: 0,
            last_line: status::TOTAL_LINES,
            handle_action: Box::new(status::STATUS_ACTIONS),
        },
        memory_list_operators: ListState {
            current_line: 0,
            first_line: 0,
            last_line: memory_list_operators_count_line,
            handle_action: Box::new(memory_list::MEMORY_LIST_ACTIONS),
        },
        memory_list_variables: ListState {
            current_line: 128,
            first_line: 128,
            last_line: 128 + memory_list_variables_count_line,
            handle_action: Box::new(memory_list::MEMORY_LIST_ACTIONS),
        },
        is_typing: false,
        typing_char: None,
        quit: false,
    };

    let mut current_state = state;

    println!("{}", clear::All);

    loop {
        terminal.draw(|mut f| {
            status::draw(&uistate, &current_state, &mut f, status_chunk);
            output::draw(&current_state, &mut f, output_chunk);
            memory_list::draw(
                &uistate,
                &current_state,
                memory_list::MemoryListKind::Operators,
                &mut f,
                memory_list_operators_chunk
            );
            memory_list::draw(
                &uistate,
                &current_state,
                memory_list::MemoryListKind::Variables,
                &mut f,
                memory_list_variables_chunk
            );
            references::draw(&mut f, references_chunk);
        })?;

        let input = action::wait_for_valid_input();
        action::execute(input, &mut current_state, &mut uistate);

        if uistate.quit {
            break Ok(())
        }
    }
}

