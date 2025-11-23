use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{
        palette::tailwind::SLATE,
        Modifier, Style, Stylize,
    },
    symbols::border,
    text::{Line, Text},
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Padding, StatefulWidget, Widget},
};
use textwrap::wrap;

use crate::core::Commit;

#[derive(Debug)]
pub struct CommitsPane<'a> {
    commits: &'a Vec<Commit>,
    has_focus: bool,
    pub state: ListState,
}

impl<'a> CommitsPane<'a> {
    pub fn new(commits: &'a Vec<Commit>, has_focus: bool, selected_commit: usize) -> Self {
        let mut pane = CommitsPane {
            commits,
            has_focus,
            state: ListState::default(),
        };

        pane.state.select(Some(selected_commit));

        pane
    }
}

impl<'a> Widget for &mut CommitsPane<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Commits ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(if self.has_focus { border::THICK } else { border::PLAIN })
            .padding(Padding::uniform(1));

        let total_width = area.width as usize;
        let horizontal_padding = 2;
        let wrap_width = total_width.saturating_sub(horizontal_padding);

        let items: Vec<ListItem> = self.commits
            .iter()
            .enumerate()
            .map(|(_, item)| {
                let mut parts = vec![
                    Line::from(""),
                    Line::from(format!(" {}", item.hash.clone())),
                    Line::from(format!(" {}", item.author.clone())),
                ];

                if let Some(msg) = &item.message {
                    for line in msg.lines() {
                        if wrap_width == 0 {
                            parts.push(Line::from(format!(" {}", line)));
                        } else {
                            for wrapped in wrap(line, wrap_width) {
                                parts.push(Line::from(format!(" {}", wrapped)));
                            }
                        }
                    }
                }

                parts.push(Line::from(""));

                ListItem::new(Text::from(parts))
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::new().bg(if self.has_focus { SLATE.c600 } else { SLATE.c700 }).add_modifier(Modifier::BOLD))
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state)
    }
}
