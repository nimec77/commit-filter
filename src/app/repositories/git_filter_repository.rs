use std::collections::HashSet;

use anyhow::Error;
use git2::DiffOptions;

use crate::app::models::{filter_results::FilterResults, git_filter_data::GitFilterData};

pub trait GitFilterRepository {
    fn filter(&self, git_filter_data: GitFilterData) -> Result<HashSet<FilterResults>, Error>;
}

pub struct GitFilterRepositoryImpl;

impl GitFilterRepository for GitFilterRepositoryImpl {
    fn filter(&self, git_filter_data: GitFilterData) -> Result<HashSet<FilterResults>, Error> {
        let repo = git_filter_data.git_repository.repository;
        let filter_path = git_filter_data.filter_path.as_ref();

        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;

        let mut result: HashSet<FilterResults> = HashSet::new();
        for oid in revwalk {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;
            let tree = commit.tree()?;

            let diff = if commit.parent_count() == 0 {
                repo.diff_tree_to_tree(None, Some(&tree), None)?
            } else {
                let parent_tree = commit.parent(0)?.tree()?;
                let mut opts = DiffOptions::new();
                repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut opts))?
            };

            for delta in diff.deltas() {
                if let Some(path) = delta.new_file().path()
                    && path.starts_with(filter_path)
                {
                    let filter_result = FilterResults::new(&commit);
                    result.insert(filter_result);
                }
            }
        }

        Ok(result)
    }
}
