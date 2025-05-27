use std::env;

use anyhow::{Context, Error};
use app::repositories::{arguments_parser_repository::{ArgumentsParserRepository, ArgumentsParserRepositoryImpl}, git_filter_repository::{GitFilterRepository, GitFilterRepositoryImpl}};

mod app;

fn main() -> Result<(), Error> {
    let args = env::args().collect::<Vec<String>>();

    let arguments_parser = ArgumentsParserRepositoryImpl;

    let filter_data = arguments_parser
        .parse_arguments(args)
        .with_context(|| "Error parsing arguments")?;

    let git_filter_repository = GitFilterRepositoryImpl;
    let filter_results = git_filter_repository.filter(filter_data)
    .with_context(|| "Error filtering commits".to_owned())?;

    for filter_result in filter_results {
        println!("{filter_result}");
    }

    Ok(())
}
