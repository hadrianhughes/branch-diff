use std::collections::HashMap;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{
        palette::tailwind::SLATE,
        Modifier, Style, Stylize,
    },
    symbols::border,
    text::{Line, Text},
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};
use textwrap::wrap;

use crate::core::Commit;

#[derive(Debug)]
pub struct CommitsPane<'a> {
    commits: &'a HashMap<String, Commit>,
    commits_order: &'a Vec<String>,
    has_focus: bool,
    pub list_state: ListState,
    pub scroll_state: ScrollbarState,
}

impl<'a> CommitsPane<'a> {
    pub fn new(
        commits: &'a HashMap<String, Commit>,
        commits_order: &'a Vec<String>,
        has_focus: bool,
        selected_commit: usize,
    ) -> Self {
        CommitsPane {
            commits,
            commits_order,
            has_focus,
            list_state: {
                let mut l = ListState::default();
                l.select(Some(selected_commit));
                l
            },
            scroll_state: ScrollbarState::new(commits.len())
                .position(selected_commit),
        }
    }
}

impl<'a> Widget for &mut CommitsPane<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Commits ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(if self.has_focus { border::THICK } else { border::PLAIN });

        let inner = block.inner(area);

        let layout_parts = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .split(inner);

        let total_width = area.width as usize;
        let horizontal_padding = 2;
        let wrap_width = total_width.saturating_sub(horizontal_padding);

        let items: Vec<ListItem> = self.commits_order
            .iter()
            .map(|hash| {
                let Some(item) = self.commits.get(hash) else {
                    panic!("No commit found for hash: {}", hash);
                };

                let mut parts = vec![
                    Line::from(""),
                    Line::from(format!(" {} ", hash.clone())),
                    Line::from(format!(" {} ", item.author.clone())),
                ];

                if let Some(msg) = &item.message {
                    for line in msg.lines() {
                        if wrap_width == 0 {
                            parts.push(Line::from(format!(" {} ", line)));
                        } else {
                            for wrapped in wrap(line, wrap_width) {
                                parts.push(Line::from(format!(" {} ", wrapped)));
                            }
                        }
                    }
                }

                parts.push(Line::from(""));

                ListItem::new(Text::from(parts))
            })
        .collect();

        block.render(area, buf);

        let list = List::new(items)
            .highlight_style(Style::new().bg(if self.has_focus { SLATE.c600 } else { SLATE.c700 }).add_modifier(Modifier::BOLD))
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, layout_parts[0], buf, &mut self.list_state);

        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .render(layout_parts[1], buf, &mut self.scroll_state)
    }
}
