use std::collections::HashMap;

use ratatui::{
    buffer::Buffer, layout::Rect, style::Stylize, text::Line, widgets::{Block, Borders, Padding, Paragraph, Widget}
};

use crate::state::{Change, ChangeKind};

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
        let mut lines = Vec::new();

        for (file_name, diff_lines) in self.file_diffs {
            lines.push(Line::from(format!("── {file_name}")).bold());

            for l in diff_lines {
                let kind_marker = match l.kind {
                    ChangeKind::Insertion => '+',
                    ChangeKind::Deletion => '-',
                    ChangeKind::Neutral => ' ',
                };

                lines.push(Line::from(format!("{} {}", kind_marker, l.text.clone())));
            }

            lines.push(Line::from(""));
        }

        let block = Block::new()
            .borders(Borders::NONE)
            .padding(Padding::uniform(1));

        Paragraph::new(lines)
            .block(block)
            .scroll((0, 0))
            .render(area, buf);
    }
}
