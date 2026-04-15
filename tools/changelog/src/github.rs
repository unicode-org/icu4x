// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::args::FetchGithub;
use indexmap::IndexMap;
use std::fs;
use std::process::Command;

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub(crate) struct GithubState {
    pub(crate) revs: IndexMap<String, PrData>,
}

impl GithubState {
    pub(crate) fn load(path: &str) -> Self {
        println!("Reading prior state from {path}");
        let file = fs::read(path).unwrap();
        serde_json::from_slice(&file).unwrap()
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub(crate) struct PrData {
    pub(crate) number: u32,
    pub(crate) body: String,
    pub(crate) title: String,
}

fn revs(revs: &str) -> Vec<String> {
    String::from_utf8(Command::new("git")
        .arg("rev-list")
        .arg("--reverse")
        .arg(revs)
        .output()
        .expect("Failed to execute `git rev-list`, make sure this is being run from an up to date ICU4X repo")
        .stdout).unwrap()
        .lines().map(|x| x.to_owned()).collect()
}

pub(crate) fn run(args: FetchGithub) {
    let revs = revs(&args.revs);

    let mut state = if args.update {
        GithubState::load(&args.json)
    } else {
        GithubState::default()
    };

    for rev in revs {
        let output = Command::new("gh")
            .arg("pr")
            .arg("list")
            .arg("--search")
            .arg(&rev)
            // Does not show merged PRs by default
            .arg("--state")
            .arg("merged")
            .arg("--json")
            .arg("title,body,number")
            .output()
            .expect("Running gh pr list failed");
        let data: Vec<PrData> = serde_json::from_slice(&output.stdout).unwrap();
        let Some(single) = data.first() else {
            println!("Found no data for rev {rev}; is it merged?");
            continue;
        };

        println!("Fetched {rev}: #{} {}", single.number, single.title);
        state.revs.insert(rev, single.clone());
    }

    println!("Writing to {}", args.json);
    let json = serde_json::to_string_pretty(&state).unwrap();
    fs::write(&args.json, json).unwrap();
    println!("Wrote to {}!", args.json);
}
