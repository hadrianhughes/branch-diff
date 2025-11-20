use ratatui::{
    buffer::Buffer, layout::Rect, style::Stylize, symbols::border, text::Line, widgets::{Block, Paragraph, Widget}
};

#[derive(Debug, Default)]
pub struct DiffPane {
    text: String,
}

impl DiffPane {
    pub fn new(text: String) -> Self {
        DiffPane { text }
    }
}

impl Widget for DiffPane {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Diff ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        Paragraph::new(self.text)
            .block(block)
            .render(area, buf)
    }
}
