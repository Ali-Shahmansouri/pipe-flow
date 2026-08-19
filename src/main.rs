use std::io::{self, stdout};

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use crate::{app::AppState, event::EventHandler};

mod action;
mod app;
mod core;
mod event;
mod ui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;

    let mut stdout = stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = AppState::new();

    let event_handler = EventHandler::new(std::time::Duration::from_millis(100));

    loop {
        terminal.draw(|frame| {
            ui::render(frame, &app);
        })?;

        if let Some(action) = event_handler.next()? {
            app.handle_action(action);
        }

        if app.should_quit() {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
