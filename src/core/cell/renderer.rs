use ratatui::{Frame, layout::Rect, widgets::Paragraph};

use crate::core::cell::visual::CellVisual;

pub trait CellRenderer {
    fn render(&mut self, visual: &CellVisual, area: RenderArea);
}

#[derive(Debug, Clone, Copy)]
pub struct RenderArea {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

pub struct RatatuiCellRenderer<'a, 'b> {
    frame: &'a mut Frame<'b>,
}

impl<'a, 'b> RatatuiCellRenderer<'a, 'b> {
    pub fn new(frame: &'a mut Frame<'b>) -> Self {
        Self { frame }
    }
}

impl<'a, 'b> CellRenderer for RatatuiCellRenderer<'a, 'b> {
    fn render(&mut self, visual: &CellVisual, area: RenderArea) {
        let area = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: area.height,
        };

        let text = visual.rows().join("\n");

        self.frame.render_widget(Paragraph::new(text), area);
    }
}
