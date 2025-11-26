use std::collections::HashMap;
use ratatui::{
    buffer::Buffer, layout::Rect, style::{Color, Style, Stylize}, symbols::border, text::Line, widgets::{Block, Borders, Padding, Paragraph, Widget}
};

use crate::state::{Change, ChangeKind};

#[derive(Debug)]
pub struct DiffPane<'a> {
    file_diffs: &'a HashMap<String, Vec<Change>>,
    scroll_position: i16,
}

impl<'a> DiffPane<'a> {
    pub fn new(file_diffs: &'a HashMap<String, Vec<Change>>, scroll_position: i16) -> Self {
        DiffPane { file_diffs, scroll_position }
    }
}

impl<'a> Widget for &DiffPane<'a> {
    /*
     * This function implements list virtualisation for scrolling.
     * Files are not rendered until the number of lines consumed is equal to the scroll position.
     * If part of the file is out of the viewport, scrolling_inside is true and only a slice of the
     * diff is rendered.
     * Rendered halts when the viewport height is full.
     * */
    fn render(self, area: Rect, buf: &mut Buffer) {
        let outer = Block::new().padding(Padding::uniform(1));

        let inner = outer.inner(area);

        outer.render(area, buf);

        let mut height_consumed: i16 = 0;
        let mut height_filled: i16 = 0;

        for (file_name, diff_lines) in self.file_diffs {
            if height_filled >= (inner.height as i16) {
                break;
            }

            let cuts_off = (diff_lines.len() as i16) + height_filled > (inner.height as i16);

            let content_height: i16 = if cuts_off {
                (inner.height as i16) - height_filled
            } else {
                diff_lines.len() as i16
            };

            height_consumed += content_height;
            if height_consumed <= self.scroll_position {
                continue;
            }

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

            let scrolling_inside = height_consumed - content_height < self.scroll_position && height_consumed >= self.scroll_position;

            let start_idx: usize = if scrolling_inside {
                content_height + self.scroll_position - height_consumed
            } else { 0 } as usize;

            let num_lines = (content_height as usize) - start_idx;

            let lines: Vec<Line> = diff_lines
                .iter()
                .skip(start_idx)
                .take(num_lines)
                .enumerate()
                .map(|(index, change)| {
                    if cuts_off && index == num_lines - 1 {
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

            // Add 2 for Block's top and bottom borders
            let outer_height = num_lines + 2;

            let space = Rect {
                x: inner.x,
                y: height_filled.try_into().expect("failed to cast height_filled as u16"),
                height: outer_height.try_into().expect("failed to cast outer_height as u16"),
                width: inner.width,
            };

            Paragraph::new(lines)
                .block(block)
                .render(space, buf);

            height_filled += outer_height as i16;
        }
    }
}
