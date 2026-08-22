use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType},
};

use crate::core::{
    board::Board,
    cell::renderer::{RatatuiCellRenderer, RenderArea},
    position::Position,
    selector::Selector,
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

pub fn render(frame: &mut Frame, board: &Board, selector: &Selector) {
    let board_area = frame.area();

    {
        let mut renderer = RatatuiCellRenderer::new(frame);

        for y in 0..board.height() {
            for x in 0..board.width() {
                let area = cell_area(board_area, x, y);
                let position = Position::new(x as usize, y as usize);

                if let Some(cell) = board.cell(position) {
                    cell.render(&mut renderer, area);
                }
            }
        }
    }

    // Render selector
    let position = selector.position();

    let area = cell_area(board_area, position.x() as u16, position.y() as u16);
    let rect = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height,
    };

    let selector_block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::Yellow));

    frame.render_widget(selector_block, rect);
}
