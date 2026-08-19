use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::action::Action;

pub struct EventHandler {
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self { tick_rate }
    }

    pub fn next(&self) -> io::Result<Option<Action>> {
        loop {
            if event::poll(self.tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    return Ok(match key.code {
                        KeyCode::Char('q') => Some(Action::Quit),

                        KeyCode::Up => Some(Action::MoveSelectorUp),
                        KeyCode::Down => Some(Action::MoveSelectorDown),
                        KeyCode::Left => Some(Action::MoveSelectorLeft),
                        KeyCode::Right => Some(Action::MoveSelectorRight),

                        KeyCode::Char(' ') => Some(Action::RotateClockwise),

                        _ => None,
                    });
                }
            }
        }
    }
}
