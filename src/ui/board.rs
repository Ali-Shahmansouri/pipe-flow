use ratatui::{Frame, layout::Rect, widgets::Block};

use crate::core::{
    board::Board,
    cell::renderer::{RatatuiCellRenderer, RenderArea},
};

const CELL_WITDTH: u16 = 5;
const CELL_HEIGHT: u16 = 3;

fn calculate_board_width(board: &Board) -> u16 {
    board.width() * CELL_WITDTH
}

fn calculate_board_height(board: &Board) -> u16 {
    board.height() * CELL_HEIGHT
}

fn cell_area(board_area: Rect, x: u16, y: u16) -> RenderArea {
    RenderArea {
        x: board_area.x + x * CELL_WITDTH,
        y: board_area.y + y * CELL_HEIGHT,
        width: CELL_WITDTH,
        height: CELL_HEIGHT,
    }
}

pub fn render(frame: &mut Frame, board: &Board) {
    let board_area = frame.area();

    let mut renderer = RatatuiCellRenderer::new(frame);

    for y in 0..board.height() {
        for x in 0..board.width() {
            let area = cell_area(board_area, x, y);

            if let Some(cell) = board.cell((x as usize, y as usize).into()) {
                cell.render(&mut renderer, area);
            }
        }
    }
}
