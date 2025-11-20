pub mod diff_pane;
pub mod files_pane;
pub mod commits_pane;

use crate::ui::{
    diff_pane::DiffPane,
    files_pane::FilesPane,
    commits_pane::CommitsPane,
};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

#[derive(Debug)]
pub struct UI {
    diff_pane: DiffPane,
    files_pane: FilesPane,
    commits_pane: CommitsPane,
}

impl UI {
    pub fn new() -> Self {
        UI {
            diff_pane: DiffPane::new("Hello world".to_string()),
            files_pane: FilesPane::default(),
            commits_pane: CommitsPane::default(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let outer_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(frame.area());

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(outer_layout[1]);

        let diff_pane = DiffPane::new("Hello world".to_string());
        let files_pane = FilesPane::default();
        let commits_pane = CommitsPane::default();

        frame.render_widget(diff_pane, outer_layout[0]);
        frame.render_widget(files_pane, right_layout[0]);
        frame.render_widget(commits_pane, right_layout[1]);
    }
}
