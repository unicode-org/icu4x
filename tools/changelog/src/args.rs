// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::{Parser, Subcommand};

/// Changelog collator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) subcommand: Sub,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Sub {
    /// Fetch PR info from GitHub, to store as JSON
    FetchGithub(FetchGithub),
    /// Fetch PR info from GitHub, to store as JSON
    MakeChangelog(MakeChangelog),
}

#[derive(Parser, Debug)]
pub(crate) struct FetchGithub {
    /// JSON file to write to
    #[arg(long = "json")]
    pub(crate) json: String,
    /// Revset to fetch
    #[arg(long = "revs")]
    pub(crate) revs: String,
    /// Update entries in an existing file. Will always append new entries to the end.
    ///
    /// Use this to update the JSON state for specific PRs (or new PRs) without having to fetch everything again
    #[arg(long = "update")]
    pub(crate) update: bool,
}

#[derive(Parser, Debug)]
pub(crate) struct MakeChangelog {
    /// JSON file to read from
    #[arg(long = "json")]
    pub(crate) json: String,
}
