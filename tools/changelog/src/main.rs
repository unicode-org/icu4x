// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use clap::Parser;

pub(crate) mod args;
pub(crate) mod changelog;
pub(crate) mod github;

fn main() {
    let args = args::Args::parse();
    match args.subcommand {
        args::Sub::FetchGithub(a) => github::run(a),
        args::Sub::MakeChangelog(a) => changelog::run(a),
    }
}
