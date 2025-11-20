use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug)]
pub struct FilesPane<'a> {
    files: &'a Vec<String>,
}

impl<'a> FilesPane<'a> {
    pub fn new(files: &'a Vec<String>) -> Self {
        FilesPane { files }
    }
}

impl<'a> Widget for &FilesPane<'a> {
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
