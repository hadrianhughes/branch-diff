use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::core::Commit;

#[derive(Debug)]
pub struct CommitsPane<'a> {
    commits: &'a Vec<Commit>,
}

impl<'a> CommitsPane<'a> {
    pub fn new(commits: &'a Vec<Commit>) -> Self {
        CommitsPane { commits }
    }
}

impl<'a> Widget for &CommitsPane<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Commits ".bold());

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        Paragraph::new("Commits go here")
            .block(block)
            .render(area, buf)
    }
}
