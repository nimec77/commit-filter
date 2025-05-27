use std::fmt::{self, Display};

use anyhow::Error;

#[derive(Debug, Clone)]
pub struct FilterPath {
    pub path: String,
}

impl FilterPath {
    pub fn parse(path: &str) -> Result<Self, Error> {
        if path.is_empty() {
            return Err(anyhow::anyhow!("Path is empty"));
        }

        Ok(Self { path: path.to_owned() })
    }
}

impl AsRef<str> for FilterPath {
    fn as_ref(&self) -> &str {
        &self.path
    }
}

impl Display for FilterPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}
