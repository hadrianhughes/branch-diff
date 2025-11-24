pub mod bottom_bar;
pub mod diff_pane;
pub mod files_pane;
pub mod commits_pane;

use crate::core::{AppState, Pane};
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
pub struct UI<'a> {
    diff_pane: DiffPane,
    files_pane: FilesPane<'a>,
    commits_pane: CommitsPane<'a>,
    bottom_bar: BottomBar<'a>,
}

impl<'a> UI<'a> {
    pub fn new(state: &'a AppState) -> Self {
        UI {
            diff_pane: DiffPane::new("Hello world".into()),
            files_pane: FilesPane::new(&state.files, matches!(state.selected_pane, Pane::Files)),
            commits_pane: CommitsPane::new(
                &state.commits,
                &state.commits_order,
                matches!(state.selected_pane, Pane::Commits),
                state.selected_commit
            ),
            bottom_bar: BottomBar::new(
                &state.from_branch,
                &state.into_branch,
            ),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
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
        frame.render_widget(&mut self.commits_pane, right_layout[1]);
        frame.render_widget(&self.bottom_bar, outer_layout[1]);
    }
}
