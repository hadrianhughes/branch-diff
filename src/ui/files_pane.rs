use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{palette::tailwind::SLATE, Color, Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};

use crate::{file_tree::{FileChangeKind, FileTree, FileTreeItem}, state::{AppState, Pane}};

#[derive(Debug, Default)]
pub struct FilesPane {}

impl StatefulWidget for &FilesPane {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = Line::from(" Files ".bold());

        let has_focus = matches!(state.selected_pane, Pane::Files);

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

        let commit = state.get_selected_commit();

        let mut lines: Vec<ListItem> = Vec::new();
        let mut selectable_indices: Vec<usize> = Vec::new();

        for (idx, FileTreeItem { node, depth }) in commit.file_tree.iter().enumerate() {
            let indent = " ".repeat(depth * 2);
            let name = match node {
                FileTree::Directory { name, .. } => format!(" / {name}"),
                FileTree::File { name, .. } => format!(" {name}"),
            };

            let style = match node {
                FileTree::Directory { .. } => Style::default(),
                FileTree::File { change_kind, .. } => match change_kind {
                    FileChangeKind::Change => Style::default(),
                    FileChangeKind::Creation => Style::default().fg(Color::Green),
                    FileChangeKind::Deletion => Style::default().fg(Color::Red),
                },
            };

            let line = Line::styled(format!("{indent}{name}"), style);
            lines.push(ListItem::new(line));

            if let FileTree::File { .. } = node {
                selectable_indices.push(idx);
            }
        }

        block.render(area, buf);

        let list = List::new(lines)
            .highlight_style(Style::new().bg(SLATE.c600).add_modifier(Modifier::BOLD))
            .highlight_spacing(HighlightSpacing::Always);

        {
            let mut list_state = ListState::default();

            if state.selected_pane == Pane::Files {
                list_state.select(Some(selectable_indices[state.selected_file]));
            }

            StatefulWidget::render(&list, layout_parts[0], buf, &mut list_state);
        }

        {
            let mut scroll_state = ScrollbarState::new(selectable_indices.len())
                .position(state.selected_file);

            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"))
                .render(layout_parts[1], buf, &mut scroll_state)
        }
    }
}
