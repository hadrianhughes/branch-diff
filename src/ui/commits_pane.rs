use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct CommitsPane {
    commits: Vec<String>,
}

impl Widget for &CommitsPane {
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
