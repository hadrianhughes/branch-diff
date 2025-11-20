pub mod bottom_bar;
pub mod diff_pane;
pub mod files_pane;
pub mod commits_pane;

use crate::ui::{
    bottom_bar::BottomBar,
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
    bottom_bar: BottomBar,
}

impl UI {
    pub fn new(from_branch: String, into_branch: String) -> Self {
        UI {
            diff_pane: DiffPane::new("Hello world".to_string()),
            files_pane: FilesPane::default(),
            commits_pane: CommitsPane::default(),
            bottom_bar: BottomBar::new(from_branch, into_branch),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(0),
                Constraint::Length(1),
            ])
            .split(frame.area());

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(outer_layout[0]);

        let right_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(main_layout[1]);

        frame.render_widget(&self.diff_pane, main_layout[0]);
        frame.render_widget(&self.files_pane, right_layout[0]);
        frame.render_widget(&self.commits_pane, right_layout[1]);
        frame.render_widget(&self.bottom_bar, outer_layout[1]);
    }
}
