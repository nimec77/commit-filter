use std::fmt::{self, Display};

use anyhow::{Context, Error};
use chrono::{DateTime, FixedOffset, TimeZone};
use git2::{Commit, Time};

#[derive(PartialEq, Eq)]
pub struct FilterResults {
    id: String,
    summary: String,
    time: Time,
}

impl std::hash::Hash for FilterResults {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl FilterResults {
    pub fn new(commit: &Commit) -> Self {
        Self {
            id: commit.id().to_string(),
            summary: commit.summary().unwrap_or_default().to_owned(),
            time: commit.time(),
        }
    }
}

impl Display for FilterResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let time_str = pretty_time(&self.time).unwrap_or_else(|_| "<invalid time>".to_owned());
        writeln!(
            f,
            "commit: {} | time: {} | summary: {}",
            short_id(&self.id),
            time_str,
            self.summary
        )?;

        Ok(())
    }
}

fn short_id(id: &str) -> String {
    id.chars().take(11).collect::<String>()
}

fn pretty_time(time: &Time) -> Result<String, Error> {
    // git2 gives offset in *minutes*; FixedOffset wants *seconds*
    let offset =
        FixedOffset::east_opt(time.offset_minutes() * 60).with_context(|| "Invalid time offset")?;

    // `timestamp_opt` avoids panics on out-of-range input
    let time_str = offset
        .timestamp_opt(time.seconds(), 0)
        .single()
        .map(|dt: DateTime<FixedOffset>| dt.format("%Y-%m-%d %H:%M:%S %z").to_string())
        .with_context(|| "Invalid time")?;

    Ok(time_str)
}
