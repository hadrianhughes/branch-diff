use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct BottomBar {
    from_branch: String,
    into_branch: String,
}

impl BottomBar {
    pub fn new(from_branch: String, into_branch: String) -> Self {
        BottomBar {
            from_branch,
            into_branch,
        }
    }
}

impl Widget for &BottomBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let from_span = Span::from(self.from_branch.as_str());
        let into_span = Span::from(self.into_branch.as_str());

        let bar = Line::from(vec![
            into_span,
            " <- ".into(),
            from_span,
        ]);

        Paragraph::new(bar).render(area, buf);
    }
}
