use git2::{DiffFormat, Repository};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::io;

use crate::core::Change;
use crate::core::ChangeKind;
use crate::core::Commit;

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
                Ok(file_diffs) => {
                    commits_order.push(hash.clone());
                    commits.insert(hash.clone(), Commit {
                        hash,
                        message,
                        author,
                        file_diffs,
                    });
                },
            }
        }

        Ok((commits, commits_order))
    }

    fn get_commit_diff(&self, commit: git2::Commit) -> Result<HashMap<String, Vec<Change>>, RepoError> {
        let tree = commit.tree()?;

        let parent_tree = if commit.parent_count() > 0 {
            Some(commit.parent(0)?.tree()?)
        } else {
            None
        };

        let diff = self.repository.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;

        let mut file_diffs: HashMap<String, Vec<Change>> = HashMap::new();

        let result = diff.print(DiffFormat::Patch, |delta, _hunk, line| {
            let Ok(text) = std::str::from_utf8(line.content()) else {
                return true;
            };

            let change_kind = match line.origin() {
                ' ' => ChangeKind::Neutral,
                '+' => ChangeKind::Insertion,
                '-' => ChangeKind::Deletion,
                _   => {
                    return true;
                },
            };

            let Some(file_path) = ({
                let path = match change_kind {
                    ChangeKind::Insertion | ChangeKind::Neutral => delta.new_file().path(),
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

            let change = Change::new(text.to_string(), change_kind);

            if let Some(file_vec) = file_diffs.get_mut(file_path) {
                file_vec.push(change);
            } else {
                let mut file_vec = Vec::new();
                file_vec.push(change);
                file_diffs.insert(file_path.to_string(), file_vec);
            }

            true
        });

        if let Err(e) = result {
            Err(RepoError::Git(e))
        } else {
            Ok(file_diffs)
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
