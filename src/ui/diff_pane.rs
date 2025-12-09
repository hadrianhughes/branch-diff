use ratatui::{
    buffer::Buffer, layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, symbols::border, text::Line, widgets::{Block, Padding, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget}
};

use crate::state::{AppState, ChangeKind, Commit};

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

        let commit = state.get_selected_commit();

        let render_area = DiffPane::render_scroll_layout(commit.diff_len, state.scroll_position, inner, buf);
        DiffPane::render_commit_diff(commit, state.scroll_position, render_area, buf);
    }
}

impl DiffPane {
    fn render_scroll_layout(diff_len: usize, scroll_position: i16, render_area: Rect, buf: &mut Buffer) -> Rect {
        if (diff_len as i16) - (render_area.height as i16) <= 0 {
            return render_area;
        }

        let layout_parts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(render_area);

        let mut scroll_state = ScrollbarState::new(((diff_len as i16) - (render_area.height as i16)) as usize)
            .position(scroll_position as usize);

        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .render(layout_parts[1], buf, &mut scroll_state);

        layout_parts[0]
    }

    fn render_commit_diff(commit: &Commit, scroll_position: i16, render_area: Rect, buf: &mut Buffer) {
        let mut rows_filled: i16 = 0;
        let mut lines_consumed: i16 = 0;
        let mut files_rendered: i16 = 0;

        for (file_name, diff_lines) in commit.file_tree.iter_files() {
            if rows_filled >= (render_area.height as i16) {
                break;
            }

            let diff_len = diff_lines.len() as i16;

            if lines_consumed + diff_len <= scroll_position {
                lines_consumed += diff_len;
                continue;
            }

            let scrolling_inside = files_rendered == 0 && scroll_position > 0;

            let start_idx = if scrolling_inside {
                std::cmp::max(scroll_position - lines_consumed, 0)
            } else { 0 };

            let truncates = if scrolling_inside {
                diff_len - start_idx + lines_consumed
            } else {
                diff_len + rows_filled
            } > (render_area.height as i16);

            let num_rows = if truncates && scrolling_inside {
                std::cmp::min(diff_len - start_idx, render_area.height as i16)
            } else if truncates {
                (render_area.height as i16) - rows_filled
            } else {
                diff_len - start_idx
            };

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
                x: render_area.x,
                y: rows_filled.try_into().expect("failed to cast height_filled as u16"),
                height: outer_height.try_into().expect("failed to cast outer_height as u16"),
                width: render_area.width,
            };

            let title = Line::from(file_name).bold();

            let block = Block::bordered()
                .title(title)
                .border_set(border::PLAIN);

            Paragraph::new(lines)
                .block(block)
                .render(space, buf);

            rows_filled += outer_height as i16;
            files_rendered += 1;
            if scrolling_inside {
                lines_consumed += diff_len - num_rows;
            }
        }
    }
}
