use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use std::io;

use crate::ui::UI;
use crate::core::AppState;

#[derive(Debug)]
pub struct App {
    state: AppState,
}

impl App {
    pub fn new(from_branch: String, into_branch: String) -> Self {
        let state = AppState::new(
            from_branch.clone(),
            into_branch.clone(),
            Vec::new(),
            Vec::new(),
        );

        App { state }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.state.exit == false {
            let ui = UI::new(&self.state);

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
            _ => {},
        }
    }
}
