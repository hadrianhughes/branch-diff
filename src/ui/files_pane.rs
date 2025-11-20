use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct FilesPane {
    files: Vec<String>,
}

impl Widget for FilesPane {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Files ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        Paragraph::new("Files go here")
            .block(block)
            .render(area, buf)
    }
}
