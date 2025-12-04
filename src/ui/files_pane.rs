use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

use crate::state::{AppState, Pane};

#[derive(Debug, Default)]
pub struct FilesPane {}

impl<'a> StatefulWidget for &'a FilesPane {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = Line::from(" Files ".bold());

        let has_focus = matches!(state.selected_pane, Pane::Files);

        let block = Block::bordered()
            .title(title.centered())
            .border_set(if has_focus { border::THICK } else { border::PLAIN });

        Paragraph::new("Files go here")
            .block(block)
            .render(area, buf)
    }
}
