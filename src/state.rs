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
    pub scroll_position: i16,
    pub scroll_height: u16,
}

#[derive(Debug)]
pub struct Commit {
    pub hash: String,
    pub message: Option<String>,
    pub author: String,
    pub file_diffs: HashMap<String, Vec<Change>>,
    pub diff_len: usize,
}

#[derive(Debug)]
pub struct Change {
    pub text: String,
    pub kind: ChangeKind,
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
            scroll_position: 0,
            scroll_height: 0,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn get_selected_commit(&self) -> &Commit {
        self.commits
            .get(self.commits_order[self.selected_commit].as_str())
            .expect(format!("attempted to get out of bounds commit with index: {}", self.selected_commit).as_str())
    }

    pub fn navigate(&mut self, direction: Direction) {
        match self.selected_pane {
            Pane::Commits => {
                match direction {
                    Direction::Down => self.select_commit(
                        if self.selected_commit == self.commits.len() - 1 { 0 } else { self.selected_commit + 1 }
                    ),
                    Direction::Up => self.select_commit(
                        if self.selected_commit == 0 { self.commits.len() - 1 } else { self.selected_commit - 1}
                    ),
                }
            },
            Pane::Diff => {
                match direction {
                    Direction::Down => {
                        let commit = self.get_selected_commit();

                        if self.scroll_position < (commit.diff_len as i16) - 1 {
                            self.scroll_position += 1;
                        }
                    },
                    Direction::Up => {
                        if self.scroll_position > 0 {
                            self.scroll_position -= 1;
                        }
                    },
                }
            },
            _ => {},
        }
    }

    pub fn select_commit(&mut self, index: usize) {
        if index < self.commits.len() {
            self.selected_commit = index;
            self.scroll_position = 0;
        } else {
            tracing::error!("attempted to select an out of bounds commit index: {index}");
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
