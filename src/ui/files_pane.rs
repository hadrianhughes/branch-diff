use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{palette::tailwind::SLATE, Modifier, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget},
};

use crate::{file_tree::FileTree, state::{AppState, Pane}};

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
        let items = render_file_tree(&commit.file_tree);

        block.render(area, buf);

        let list = List::new(items)
            .highlight_style(Style::new().bg(SLATE.c600).add_modifier(Modifier::BOLD))
            .highlight_spacing(HighlightSpacing::Always);

        {
            let mut list_state = ListState::default();

            if state.selected_pane == Pane::Files {
                list_state.select(Some(state.selected_file));
            }

            StatefulWidget::render(&list, layout_parts[0], buf, &mut list_state);
        }

        {
            let mut scroll_state = ScrollbarState::new(list.len())
                .position(state.selected_file);

            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"))
                .render(layout_parts[1], buf, &mut scroll_state)
        }
    }
}

fn render_file_tree<'a>(file_tree: &'a FileTree) -> Vec<ListItem<'a>> {
    let mut file_tree_iter = file_tree.iter();
    let mut lines: Vec<ListItem> = Vec::new();

    while let Some((node, depth)) = file_tree_iter.next() {
        let indent = " ".repeat(depth * 2);
        let name = match node {
            FileTree::Directory { name, .. } => format!(" / {name}"),
            FileTree::File { name, .. } => format!(" {name}"),
        };

        lines.push(ListItem::new(format!("{indent}{name}")));
    }

    return lines;
}
