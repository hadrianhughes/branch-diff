use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::state::AppState;

#[derive(Debug, Default)]
pub struct BottomBar {}

impl<'a> StatefulWidget for &BottomBar {
    type State = AppState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let from_span = Span::from(state.from_branch.as_str());
        let into_span = Span::from(state.into_branch.as_str());

        let bar = Line::from(vec![
            into_span,
            " <- ".into(),
            from_span,
        ]);

        Paragraph::new(bar).render(area, buf);
    }
}
