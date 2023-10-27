use crossterm::{
    event::{self, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::io::{stderr, Result};
use ui::Component;

mod state;
use state::{Action, State};

mod cli;
mod keyboard;
use keyboard::Keyboard;
mod ui;
mod utils;

fn main() -> Result<()> {
    let target = cli::main();
    let keyboard = Keyboard::new(
        "/Users/jimzer/Projects/bitswired/rust-test/typing_game/layout.txt".to_string(),
    )
    .unwrap();
    let mut state = State::new(keyboard);
    state.dispatch(Action::SetTarget(target.unwrap()));

    // println!("{:?}", state.keyboard.touch_map);
    // return Ok(());

    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    let mut type_test_view_component = ui::TypeTestView::new();

    loop {
        state.type_test.keyboard.tick();

        terminal.draw(|frame| {
            if let state::View::TypeTest = state.current_view {
                type_test_view_component.render(&state, frame, frame.size())
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match (key.modifiers, key.code) {
                    (event::KeyModifiers::CONTROL, _) => break, // This will catch Control + C and break out of the loop
                    (_, KeyCode::Char(k)) => {
                        state.dispatch(Action::Char(k));
                    }
                    (_, KeyCode::Backspace) => state.dispatch(Action::Backspace), // Check for backspace and dispatch the action
                    (_, KeyCode::Enter) => state.dispatch(Action::Char('\n')), // Check for backspace and dispatch the action
                    _ => {} // For all other keys, do nothing
                }
            }
        }
    }

    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
