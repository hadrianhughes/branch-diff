use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use core::fmt;
use std::io;

use crate::ui::UI;
use crate::state::{AppState, Direction, Pane};
use crate::repo::{Repo, RepoError};

#[derive(Debug)]
pub struct App {
    state: AppState,
    repository: Repo,
}

#[derive(Debug)]
pub enum AppError {
    Repo(RepoError),
    NoCommits,
}

impl App {
    pub fn new(repository: Repo, from_branch: String, into_branch: String) -> Result<Self, AppError> {
        let (commits, commits_order) = repository.commits_in_range(
            into_branch.as_str(),
            from_branch.as_str(),
        )?;

        if commits.is_empty() {
            return Err(AppError::NoCommits)
        }

        let state = AppState::new(
            from_branch.clone(),
            into_branch.clone(),
            commits,
            commits_order,
        );

        Ok(App { state, repository })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state.exit == false {
            let mut ui = UI::default();

            terminal.draw(|frame| ui.render(frame, &mut self.state))?;

            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.state.exit(),
            KeyCode::Char('j') | KeyCode::Down => self.state.navigate(Direction::Down),
            KeyCode::Char('k') | KeyCode::Up => self.state.navigate(Direction::Up),
            KeyCode::Char('c') => self.state.select_pane(Pane::Commits),
            KeyCode::Char('d') => self.state.select_pane(Pane::Diff),
            KeyCode::Char('f') => self.state.select_pane(Pane::Files),
            KeyCode::Enter => self.state.select(),
            _ => {},
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Repo(e) => write!(f, "AppError: {}", e),
            AppError::NoCommits => write!(f, "no commits to display"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Repo(e) => Some(e),
            AppError::NoCommits => None,
        }
    }
}

impl From<RepoError> for AppError {
    fn from(e: RepoError) -> Self {
        AppError::Repo(e)
    }
}
