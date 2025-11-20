#[derive(Debug)]
pub struct AppState {
    pub exit: bool,
    pub from_branch: String,
    pub into_branch: String,
    pub commits: Vec<String>,
    pub files: Vec<String>,
}

impl AppState {
    pub fn new(
        from_branch: String,
        into_branch: String,
        commits: Vec<String>,
        files: Vec<String>,
    ) -> Self {
        AppState {
            from_branch,
            into_branch,
            exit: false,
            commits,
            files,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
