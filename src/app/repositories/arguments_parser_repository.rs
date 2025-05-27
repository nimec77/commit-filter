use anyhow::{Context, Error};

use crate::app::models::{filter_path::FilterPath, get_repository::GitRepository, git_filter_data::GitFilterData};

pub trait ArgumentsParserRepository {
    fn parse_arguments(&self, arguments: Vec<String>) -> Result<GitFilterData, Error>;
}

pub struct ArgumentsParserRepositoryImpl;

impl ArgumentsParserRepository for ArgumentsParserRepositoryImpl {
    fn parse_arguments(&self, arguments: Vec<String>) -> Result<GitFilterData, Error> {
        let repo_path = arguments.get(1).with_context(|| "Repo path not found")?;
        let raw_filter_path = arguments.get(2).with_context(|| "Filter path not found")?;

        let filter_path = FilterPath::parse(raw_filter_path)
            .with_context(|| format!("Error parsing filter path: {raw_filter_path}"))?;

        let git_repository = GitRepository::parse(repo_path)
            .with_context(|| format!("Error parsing git repository: {repo_path}"))?;

        Ok(GitFilterData {
            git_repository,
            filter_path,
        })
    }
}
