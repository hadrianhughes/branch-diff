use ratatui::{
    buffer::Buffer, layout::Rect, widgets::{Block, Borders, Padding, Paragraph, Widget}
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

impl Widget for &DiffPane {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .padding(Padding::uniform(1))
            .borders(Borders::NONE);

        Paragraph::new(self.text.as_str())
            .block(block)
            .render(area, buf)
    }
}
