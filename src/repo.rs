use git2::{DiffFormat, Repository};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::io;

use crate::file_tree::FileChangeKind;
use crate::file_tree::FileTree;
use crate::file_tree::FileTreeFilesItem;
use crate::state::Change;
use crate::state::ChangeKind;
use crate::state::Commit;

pub struct Repo {
    repository: Repository,
}

impl Repo {
    pub fn new() -> Result<Self, RepoError> {
        let path = env::current_dir()?;
        let repository = Repository::open(path)?;

        Ok(Repo { repository })
    }

    pub fn commits_in_range(&self, base: &str, head: &str) -> Result<(HashMap<String, Commit>, Vec<String>), RepoError> {
        let base_ref = self.repository.revparse_single(base)?;
        let head_ref = self.repository.revparse_single(head)?;

        let mut revwalk = self.repository.revwalk()?;
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)?;

        revwalk.hide(base_ref.id())?;
        revwalk.push(head_ref.id())?;

        let mut commits = HashMap::new();
        let mut commits_order = Vec::new();

        for oid in revwalk {
            let commit = self.repository.find_commit(oid?)?;
            let hash = commit.id().to_string();
            let author = commit.author().to_string();
            let message = match commit.message() {
                Some(msg) => Some(msg.to_string()),
                None => None,
            };

            match self.get_commit_diff(commit) {
                Err(e) => {
                    return Err(e);
                },
                Ok(file_tree) => {
                    let diff_len = file_tree
                        .iter_files()
                        .map(|FileTreeFilesItem { changes, .. }| { changes.len() })
                        .sum();

                    commits_order.push(hash.clone());
                    commits.insert(hash.clone(), Commit {
                        hash,
                        message,
                        author,
                        file_tree,
                        diff_len,
                    });
                },
            }
        }

        Ok((commits, commits_order))
    }

    fn get_commit_diff(&self, commit: git2::Commit) -> Result<FileTree, RepoError> {
        let tree = commit.tree()?;

        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let diff = self.repository.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;

        let root_dir = match self.repository.workdir() {
            Some(dir) => dir
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("root"),
            None => "root",
        };

        let mut file_tree = FileTree::new(root_dir);
        let mut previous_file_path: Option<String> = None;
        let mut previous_file_diff: Vec<Change> = Vec::new();
        let mut previous_change_kind: Option<FileChangeKind> = None;
        let mut next_scroll_start: usize = 0;
        let mut line_count: usize = 0;

        let result = diff.print(DiffFormat::Patch, |delta, _hunk, line| {
            let Ok(text) = std::str::from_utf8(line.content()) else {
                return true;
            };

            let change_kind = match line.origin() {
                ' ' => ChangeKind::Context,
                '+' => ChangeKind::Insertion,
                '-' => ChangeKind::Deletion,
                _   => {
                    return true;
                },
            };

            let Some(file_path) = ({
                let path = match change_kind {
                    ChangeKind::Insertion | ChangeKind::Context => delta.new_file().path(),
                    ChangeKind::Deletion => delta.old_file().path(),
                };

                if let Some(p) = path {
                    p.to_str()
                } else {
                    None
                }
            }) else {
                println!("Couldn't get file path for diff delta");
                return true;
            };

            if let Some(cfp) = &previous_file_path && file_path != cfp {
                file_tree.insert_file(
                    cfp.as_str(),
                    std::mem::take(&mut previous_file_diff),
                    previous_change_kind.expect("previous_change_kind was None when trying to insert file"),
                    next_scroll_start,
                );

                previous_file_diff.clear();
                previous_change_kind = None;
                next_scroll_start = line_count;
            }

            previous_file_path = Some(file_path.to_string());
            previous_change_kind = Some(Repo::update_file_change_kind(previous_change_kind, change_kind));
            line_count += 1;

            let change = Change {
                text: text.to_string(),
                kind: change_kind,
            };

            previous_file_diff.push(change);

            true
        });

        file_tree.insert_file(
            previous_file_path.expect("previous_file_path was None when trying to insert file").as_str(),
            previous_file_diff,
            previous_change_kind.expect("previous_change_kind was None when trying to insert file"),
            next_scroll_start,
        );

        if let Err(e) = result {
            Err(RepoError::Git(e))
        } else {
            file_tree.sort();
            Ok(file_tree)
        }
    }

    fn update_file_change_kind(file_change_kind: Option<FileChangeKind>, line_change_kind: ChangeKind) -> FileChangeKind {
        match file_change_kind {
            Some(kind) => match kind {
                FileChangeKind::Creation => match line_change_kind {
                    ChangeKind::Context | ChangeKind::Deletion => FileChangeKind::Change,
                    ChangeKind::Insertion => FileChangeKind::Creation,
                },
                FileChangeKind::Deletion => match line_change_kind {
                    ChangeKind::Context | ChangeKind::Insertion => FileChangeKind::Change,
                    ChangeKind::Deletion => FileChangeKind::Deletion,
                },
                FileChangeKind::Change => FileChangeKind::Change,
            },
            None => match line_change_kind {
                ChangeKind::Context => FileChangeKind::Change,
                ChangeKind::Insertion => FileChangeKind::Creation,
                ChangeKind::Deletion => FileChangeKind::Deletion,
            },
        }
    }
}

impl std::fmt::Debug for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.repository.path().to_str() {
            Some(p) => write!(f, "Repo <{}>", p),
            None => write!(f, "Repo"),
        }
    }
}

#[derive(Debug)]
pub enum RepoError {
    Io(io::Error),
    Git(git2::Error),
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepoError::Io(e) => write!(f, "IO error: {}", e),
            RepoError::Git(e) => write!(f, "Git error: {}", e),
        }
    }
}

impl std::error::Error for RepoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RepoError::Io(e) => Some(e),
            RepoError::Git(e) => Some(e),
        }
    }
}

impl From<io::Error> for RepoError {
    fn from(e: io::Error) -> Self {
        RepoError::Io(e)
    }
}

impl From<git2::Error> for RepoError {
    fn from(e: git2::Error) -> Self {
        RepoError::Git(e)
    }
}
