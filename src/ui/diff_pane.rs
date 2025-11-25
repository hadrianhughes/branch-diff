use std::collections::HashMap;

use ratatui::{
    buffer::Buffer, layout::Rect, widgets::{Block, Borders, Padding, Paragraph, Widget}
};

use crate::state::Change;

#[derive(Debug)]
pub struct DiffPane<'a> {
    file_diffs: &'a HashMap<String, Vec<Change>>,
}

impl<'a> DiffPane<'a> {
    pub fn new(file_diffs: &'a HashMap<String, Vec<Change>>) -> Self {
        DiffPane { file_diffs }
    }
}

impl<'a> Widget for &DiffPane<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .padding(Padding::uniform(1))
            .borders(Borders::NONE);

        Paragraph::new("Diff here")
            .block(block)
            .render(area, buf)
    }
}
