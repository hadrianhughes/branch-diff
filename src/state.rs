use std::collections::HashMap;

#[derive(Debug)]
pub struct AppState {
    pub exit: bool,
    pub from_branch: String,
    pub into_branch: String,
    pub commits: HashMap<String, Commit>,
    pub commits_order: Vec<String>,
    pub files: Vec<String>,
    pub selected_pane: Pane,
    pub selected_commit: usize,
}

#[derive(Debug)]
pub struct Commit {
    pub hash: String,
    pub message: Option<String>,
    pub author: String,
    pub file_diffs: HashMap<String, Vec<Change>>,
}

#[derive(Debug)]
pub struct Change {
    text: String,
    kind: ChangeKind,
}

impl Change {
    pub fn new(text: String, kind: ChangeKind) -> Self {
        Change { text, kind }
    }
}

#[derive(Debug)]
pub enum ChangeKind {
    Neutral = 0,
    Insertion = 1,
    Deletion = 2,
}

#[derive(Debug)]
pub enum Pane {
    Diff = 0,
    Files = 1,
    Commits = 2,
}

pub enum Direction {
    Down = 0,
    Up = 1,
    Left = 2,
    Right = 3,
}

impl AppState {
    pub fn new(
        from_branch: String,
        into_branch: String,
        commits: HashMap<String, Commit>,
        commits_order: Vec<String>,
        files: Vec<String>,
    ) -> Self {
        AppState {
            from_branch,
            into_branch,
            exit: false,
            commits,
            commits_order,
            files,
            selected_pane: Pane::Diff,
            selected_commit: 0,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn navigate(&mut self, direction: Direction) {
        match self.selected_pane {
            Pane::Commits => {
                match direction {
                    Direction::Down => {
                        self.selected_commit = if self.selected_commit == self.commits.len() - 1 { 0 } else { self.selected_commit + 1 };
                    }
                    Direction::Up => {
                        self.selected_commit = if self.selected_commit == 0 { self.commits.len() - 1 } else { self.selected_commit - 1};
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    pub fn select_pane(&mut self, pane: Pane) {
        self.selected_pane = pane;
    }

    pub fn select(&mut self) {
        match self.selected_pane {
            Pane::Commits => {
                self.select_pane(Pane::Diff);
            },
            _ => {},
        }
    }
}
