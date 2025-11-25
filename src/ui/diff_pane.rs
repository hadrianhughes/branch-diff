use std::collections::HashMap;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget},
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
        let constraints = self.file_diffs.iter().map(|(_, lines)| {
            Constraint::Length(lines.len() as u16 + 2)
        }).collect::<Vec<_>>();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        for (i, (key, _)) in self.file_diffs.iter().enumerate() {
            let file_title = Line::from(format!(" {} ", key));

            let file_block = Block::bordered()
                .title(file_title)
                .padding(Padding::uniform(1))
                .border_set(border::PLAIN);

            let paragraph = Paragraph::new("Diff contents")
                .block(file_block);

            paragraph.render(chunks[i], buf);
        }
    }
}
