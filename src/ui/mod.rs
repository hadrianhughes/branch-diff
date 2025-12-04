pub mod bottom_bar;
pub mod diff_pane;
pub mod files_pane;
pub mod commits_pane;

use crate::state::AppState;
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

#[derive(Debug, Default)]
pub struct UI {
    diff_pane: DiffPane,
    files_pane: FilesPane,
    commits_pane: CommitsPane,
    bottom_bar: BottomBar,
}

impl<'a> UI {
    pub fn render(&mut self, frame: &mut Frame, state: &mut AppState) {
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

        frame.render_stateful_widget(&self.diff_pane, main_layout[0], state);
        frame.render_stateful_widget(&self.files_pane, right_layout[0], state);
        frame.render_stateful_widget(&self.commits_pane, right_layout[1], state);
        frame.render_stateful_widget(&self.bottom_bar, outer_layout[1], state);
    }
}
