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
    has_focus: bool,
}

impl<'a> FilesPane<'a> {
    pub fn new(files: &'a Vec<String>, has_focus: bool) -> Self {
        FilesPane { files, has_focus }
    }
}

impl<'a> Widget for &FilesPane<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Files ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(if self.has_focus { border::THICK } else { border::PLAIN });

        Paragraph::new("Files go here")
            .block(block)
            .render(area, buf)
    }
}
