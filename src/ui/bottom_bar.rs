use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

#[derive(Debug)]
pub struct BottomBar<'a> {
    from_branch: &'a String,
    into_branch: &'a String,
}

impl<'a> BottomBar<'a> {
    pub fn new(from_branch: &'a String, into_branch: &'a String) -> Self {
        BottomBar {
            from_branch,
            into_branch,
        }
    }
}

impl<'a> Widget for &BottomBar<'a> {
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
