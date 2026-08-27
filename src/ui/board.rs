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

const CELL_SLOT_WITDTH: u16 = 7;
const CELL_SLOT_HEIGHT: u16 = 5;

const CELL_WIDTH: u16 = 5;
const CELL_HEIGHT: u16 = 3;

impl From<RenderArea> for Rect {
    fn from(area: RenderArea) -> Self {
        Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height,
        }
    }
}

fn cell_area(board_area: Rect, x: u16, y: u16) -> RenderArea {
    RenderArea {
        x: board_area.x + x * CELL_SLOT_WITDTH,
        y: board_area.y + y * CELL_SLOT_HEIGHT,
        width: CELL_SLOT_WITDTH,
        height: CELL_SLOT_HEIGHT,
    }
}

fn cell_content_area(area: RenderArea) -> RenderArea {
    RenderArea {
        x: area.x + 1,
        y: area.y + 1,
        width: CELL_WIDTH,
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
                let content_area = cell_content_area(area);

                let position = Position::new(x as usize, y as usize);

                if let Some(cell) = board.cell(position) {
                    cell.render(&mut renderer, content_area);
                }
            }
        }
    }

    // Source | Destination
    render_endpoint(
        frame,
        board_area,
        board.start_position(),
        Color::Green,
        BorderType::Rounded,
    );

    render_endpoint(
        frame,
        board_area,
        board.destination_position(),
        Color::Cyan,
        BorderType::Double,
    );

    // Render selector
    let position = selector.position();

    let area = cell_area(board_area, position.x() as u16, position.y() as u16);

    let selector_block = Block::bordered()
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::Yellow));

    frame.render_widget(selector_block, area.into());
}

fn render_endpoint(
    frame: &mut Frame,
    board_area: Rect,
    position: Position,
    color: Color,
    border_type: BorderType,
) {
    let area = cell_area(board_area, position.x() as u16, position.y() as u16);

    let block = Block::bordered()
        .border_type(border_type)
        .border_style(Style::default().fg(color));

    frame.render_widget(block, area.into());
}
