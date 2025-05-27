use std::fmt::{self, Display};

use anyhow::Error;
use git2::Repository;

pub struct GitRepository {
    pub path: String,
    pub repository: Repository,
}

impl GitRepository {
    pub fn parse(path: &str) -> Result<Self, Error> {
        let repo = Repository::open(path)?;

        Ok(Self {
            path: path.to_owned(),
            repository: repo,
        })
    }
}

impl Display for GitRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
