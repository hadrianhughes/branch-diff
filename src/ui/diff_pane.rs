use ratatui::{
    buffer::Buffer, layout::Rect, style::{Color, Style, Stylize}, symbols::border, text::Line, widgets::{Block, Padding, Paragraph, StatefulWidget, Widget}
};

use crate::state::{AppState, ChangeKind};

#[derive(Debug, Default)]
pub struct DiffPane {}

impl<'a> StatefulWidget for &'a DiffPane {
    type State = AppState;

    /*
     * This function implements list virtualisation for scrolling.
     * Files are not rendered until the number of lines consumed is equal to the scroll position.
     * If only part of a file is out of the viewport, scrolling_inside is true and only the
     * remainder of the file is rendered.
     * Rendered halts when the viewport height is full.
     *
     * Lines are defined to be the actual lines of content in the diffs.
     * Rows are the visual lines in the UI that are rendered into.
     * */
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let outer = Block::new().padding(Padding::uniform(1));
        let inner = outer.inner(area);

        state.scroll_height = inner.height as i16;

        outer.render(area, buf);

        let mut rows_filled: i16 = 0;
        let mut lines_consumed: i16 = 0;
        let mut files_rendered: i16 = 0;

        let commit = state.get_selected_commit();

        for (file_name, diff_lines) in &commit.file_diffs {
            if rows_filled >= (inner.height as i16) {
                break;
            }

            let diff_len = diff_lines.len() as i16;

            let truncates = diff_len + rows_filled > (inner.height as i16);

            let content_height: i16 = if truncates {
                (inner.height as i16) - rows_filled
            } else { diff_len };

            if lines_consumed + content_height <= state.scroll_position {
                lines_consumed += content_height;
                continue;
            }

            let title = Line::from(file_name.clone()).bold();

            let block = Block::bordered()
                .title(title)
                .border_set(border::PLAIN);

            let scrolling_inside = files_rendered == 0 && state.scroll_position > 0;
            let start_idx = if scrolling_inside {
                std::cmp::max(state.scroll_position - lines_consumed, 0)
            } else { 0 };

            let num_rows = content_height - start_idx;

            let lines: Vec<Line> = diff_lines
                .iter()
                .skip(start_idx as usize)
                .take(num_rows as usize)
                .map(|change| {
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
            let outer_height = num_rows + 2;

            let space = Rect {
                x: inner.x,
                y: rows_filled.try_into().expect("failed to cast height_filled as u16"),
                height: outer_height.try_into().expect("failed to cast outer_height as u16"),
                width: inner.width,
            };

            Paragraph::new(lines)
                .block(block)
                .render(space, buf);

            rows_filled += outer_height as i16;
            files_rendered += 1;
            if scrolling_inside {
                lines_consumed += content_height - num_rows;
            }
        }
    }
}
