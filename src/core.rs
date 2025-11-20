#[derive(Debug)]
pub struct AppState {
    pub from_branch: String,
    pub into_branch: String,
    pub exit: bool,
}

impl AppState {
    pub fn new(from_branch: String, into_branch: String) -> Self {
        AppState {
            from_branch,
            into_branch,
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
