use std::fmt::{self, Display};

use super::{filter_path::FilterPath, get_repository::GitRepository};

pub struct GitFilterData {
    pub git_repository: GitRepository,
    pub filter_path: FilterPath,
}

impl Display for GitFilterData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.git_repository, self.filter_path)
    }
}
