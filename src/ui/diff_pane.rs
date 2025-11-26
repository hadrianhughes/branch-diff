use std::collections::HashMap;

use ratatui::{
    buffer::Buffer, layout::Rect, style::{Color, Style, Stylize}, symbols::border, text::Line, widgets::{Block, Borders, Padding, Paragraph, Widget}
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
        let outer = Block::new().padding(Padding::uniform(1));

        let inner = outer.inner(area);

        outer.render(area, buf);

        let mut height_used: u16 = 0;

        for (file_name, diff_lines) in self.file_diffs {
            let mut cuts_off = false;

            if height_used >= inner.height {
                break;
            }

            let content_height: u16 = if (diff_lines.len() as u16) + height_used > inner.height {
                cuts_off = true;
                inner.height - height_used
            } else {
                diff_lines.len() as u16
            };

            let outer_height = content_height + 2;

            let title = Line::from(file_name.clone()).bold();

            let block = Block::bordered()
                .title(title)
                .border_set(border::PLAIN)
                .borders(
                    if cuts_off {
                        Borders::TOP | Borders::LEFT | Borders::RIGHT
                    } else {
                        Borders::ALL
                    }
                );

            let lines: Vec<Line> = diff_lines
                .iter()
                .take(content_height as usize)
                .enumerate()
                .map(|(index, change)| {
                    if cuts_off && (index as u16) == content_height - 1 {
                        return Line::from("  ...");
                    }

                    let prefix = match change.kind {
                        ChangeKind::Neutral => ' ',
                        ChangeKind::Insertion => '+',
                        ChangeKind::Deletion => '-',
                    };

                    let style = match change.kind {
                        ChangeKind::Neutral => Style::default(),
                        ChangeKind::Insertion => Style::default()
                            .fg(Color::Green),
                        ChangeKind::Deletion => Style::default()
                            .fg(Color::Red),
                    };

                    Line::styled(format!("{prefix} {}", change.text.clone()), style)
                })
                .collect();

            let space = Rect {
                x: inner.x,
                y: height_used,
                height: outer_height,
                width: inner.width,
            };

            height_used += outer_height;

            Paragraph::new(lines)
                .block(block)
                .render(space, buf);
        }
    }
}
