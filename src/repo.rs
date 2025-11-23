use std::env;
use std::fmt;
use std::io;

use git2::Repository;

pub struct Repo {
    repository: Repository,
}

impl Repo {
    pub fn new() -> Result<Self, RepoError> {
        let path = env::current_dir()?;
        let repository = Repository::open(path)?;

        Ok(Repo { repository })
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
