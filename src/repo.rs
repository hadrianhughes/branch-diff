use git2::Repository;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::io;

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

    pub fn commits_in_range(&self, base: &str, head: &str) -> Result<Vec<Commit>, RepoError> {
        let base_ref = self.repository.revparse_single(base)?;
        let head_ref = self.repository.revparse_single(head)?;

        let mut revwalk = self.repository.revwalk()?;
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL | git2::Sort::REVERSE)?;

        revwalk.hide(base_ref.id())?;
        revwalk.push(head_ref.id())?;

        let mut commits = Vec::new();
        for oid in revwalk {
            let commit = self.repository.find_commit(oid?)?;
            commits.push(Commit {
                hash: commit.id().to_string(),
                message: match commit.message() {
                    Some(msg) => Some(msg.to_string()),
                    None => None,
                },
                author: commit.author().to_string(),
                diff: HashMap::new(),
            });
        }

        Ok(commits)
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
