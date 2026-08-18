use ratatui::Frame;

use crate::app::AppState;

mod board;

pub fn render(frame: &mut Frame, app: &AppState) {
    board::render(frame, app.board());
}
