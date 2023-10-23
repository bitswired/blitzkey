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
mod ui;

fn main() -> Result<()> {
    let target = cli::main();
    let mut state = State::new();
    state.dispatch(Action::SetTarget(target.unwrap()));

    stderr().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stderr()))?;
    terminal.clear()?;

    let typeTestViewComponent = ui::TypeTestView::new();

    loop {
        terminal.draw(|frame| {
            match &state.current_view {
                state::View::TypeTest => typeTestViewComponent.render(&state, frame, frame.size()),
                _ => {}
            };
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(key) = event::read()? {
                match (key.modifiers, key.code) {
                    (event::KeyModifiers::CONTROL, _) => break, // This will catch Control + C and break out of the loop
                    (_, KeyCode::Char(k)) => {
                        state.dispatch(Action::Char(k));
                    }
                    (_, KeyCode::Backspace) => state.dispatch(Action::Backspace), // Check for backspace and dispatch the action
                    (_, KeyCode::Enter) => state.dispatch(Action::Enter), // Check for backspace and dispatch the action
                    _ => {} // For all other keys, do nothing
                }
            }
        }
    }

    stderr().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
