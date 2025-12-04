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

use crate::state::{AppState, Pane};

#[derive(Debug, Default)]
pub struct CommitsPane {}

impl<'a> StatefulWidget for &'a CommitsPane {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = Line::from(" Commits ".bold());

        let has_focus = matches!(state.selected_pane, Pane::Commits);

        let block = Block::bordered()
            .title(title.centered())
            .border_set(if has_focus { border::THICK } else { border::PLAIN });

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

        let items: Vec<ListItem> = state.commits_order
            .iter()
            .map(|hash| {
                let Some(item) = state.commits.get(hash) else {
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
            .highlight_style(Style::new().bg(if has_focus { SLATE.c600 } else { SLATE.c700 }).add_modifier(Modifier::BOLD))
            .highlight_spacing(HighlightSpacing::Always);

        {
            let mut list_state = ListState::default();
            list_state.select(Some(state.selected_commit));
            StatefulWidget::render(list, layout_parts[0], buf, &mut list_state);
        }

        {
            let mut scroll_state = ScrollbarState::new(state.commits.len())
                .position(state.selected_commit);

            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"))
                .render(layout_parts[1], buf, &mut scroll_state)
        }
    }
}
