use ratatui::{Frame, layout::Rect, widgets::Block};

use crate::core::board::Board;

const CELL_WITDTH: u16 = 5;
const CELL_HEIGHT: u16 = 3;

fn calculate_board_width(board: &Board) -> u16 {
    board.width() * CELL_WITDTH
}

fn calculate_board_height(board: &Board) -> u16 {
    board.height() * CELL_HEIGHT
}

fn cell_area(boarad_area: Rect, x: u16, y: u16) -> Rect {
    Rect {
        x: boarad_area.x + x * CELL_WITDTH,
        y: boarad_area.y + y * CELL_HEIGHT,
        width: CELL_WITDTH,
        height: CELL_HEIGHT,
    }
}

pub fn render(frame: &mut Frame, board: &Board) {
    let board_area = frame.area();

    for y in 0..board.height() {
        for x in 0..board.width() {
            let area = cell_area(board_area, x, y);

            let cell = Block::bordered();

            frame.render_widget(cell, area);
        }
    }
}
