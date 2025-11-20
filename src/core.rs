#[derive(Debug)]
pub struct AppState {
    pub exit: bool,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
