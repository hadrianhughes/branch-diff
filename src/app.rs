use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

use crate::ui::UI;
use crate::state::{AppState, Direction, Pane};
use crate::repo::Repo;

#[derive(Debug)]
pub struct App {
    state: AppState,
    repository: Repo,
}

impl App {
    pub fn new(repository: Repo, from_branch: String, into_branch: String) -> Self {
        let (commits, commits_order) = match repository.commits_in_range(into_branch.as_str(), from_branch.as_str()) {
            Ok(cs) => cs,
            Err(e) => panic!("Couldn't get commits: {}", e),
        };

        let state = AppState::new(
            from_branch.clone(),
            into_branch.clone(),
            commits,
            commits_order,
            Vec::new(),
        );

        App { state, repository }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state.exit == false {
            let mut ui = UI::new(&self.state);

            terminal.draw(|frame| ui.render(frame))?;

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
            KeyCode::Enter => if matches!(self.state.selected_pane, Pane::Commits) {
                self.state.select_pane(Pane::Diff);
            }
            _ => {},
        }
    }
}
