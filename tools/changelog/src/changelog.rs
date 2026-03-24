// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::args::MakeChangelog;
use crate::github::{GithubState, PrData};
use regex::Regex;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::sync::LazyLock;

pub(crate) fn run(args: MakeChangelog) {
    let state = GithubState::load(&args.json);
    let mut organized = OrganizedChangelog::default();
    for data in state.revs.values() {
        organized.add(data);
    }

    organized.render();
}

#[derive(Debug, Default)]
struct OrganizedChangelog {
    /// Different crate sections
    sections: BTreeMap<String, Vec<ChangelogEntry>>,
    /// Additional data that was not included in the crate sections.
    additional: Vec<(PrData, String)>,
    /// N/A PRs
    n_a: Vec<PrData>,
    no_changelog_found: Vec<PrData>,
    misformatted: Vec<PrData>,
}

#[derive(Debug, Default)]
struct ChangelogEntry {
    number: u32,
    entry: String,
    /// Sub bullet points, with indent level.
    bullets: Vec<(usize, String)>,
}

static CHANGELOG_HEADER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("(\n|^)#+ Changelog(?<annotation>.*(\n|$))").unwrap());
static SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^(?<crate>\\S+):(?<entry>.*)$").unwrap());

#[derive(Clone, Debug, Default)]
struct SectionState {
    krate: String,
    entry: String,
    bullets: Vec<(usize, String)>,
    indent_stack: Vec<usize>,
}

impl OrganizedChangelog {
    fn add(&mut self, data: &PrData) {
        let Some(header) = CHANGELOG_HEADER.captures(&data.body) else {
            self.no_changelog_found.push(data.clone());
            return;
        };

        if header.name("annotation").unwrap().as_str().contains("N/A") {
            self.n_a.push(data.clone());
            return;
        }

        let changelog_data = CHANGELOG_HEADER.split(&data.body).nth(1).unwrap();

        let mut current_section: Option<SectionState> = None;
        let mut additional_lines = String::new();
        for line in changelog_data.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            if let Some(header) = SECTION.captures(line) {
                self.flush(data, &mut current_section);
                let entry = header.name("entry").unwrap().as_str().trim().to_owned();

                current_section = Some(SectionState {
                    krate: header.name("crate").unwrap().as_str().to_owned(),
                    entry,
                    bullets: Vec::new(),
                    indent_stack: Vec::new(),
                })
            } else if trimmed.starts_with('-') || trimmed.starts_with('*') {
                if let Some(current_section) = current_section.as_mut() {
                    let bullet_index = line.find(['-', '*']).unwrap();
                    let bullet_line = line[bullet_index + 1..].trim().to_owned();

                    // Occasionally people misformat changelog entries
                    // by writing `icu_foo:` and then following it by a bulleted list.
                    //
                    // We just make a new entry for each sub bullet there.
                    if current_section.entry.is_empty() {
                        // clone instead of flushing if there are multiple bullets
                        let mut section_to_append = current_section.clone();
                        section_to_append.entry = bullet_line;
                        self.append_section(data, section_to_append);
                        continue;
                    }

                    let nth = current_section.indent_stack.binary_search(&bullet_index);
                    let idx = match nth {
                        Ok(idx) => {
                            // If there are indents beyond this, skip them
                            current_section.indent_stack.truncate(idx + 1);
                            idx
                        }
                        Err(idx) => {
                            // If there are indents beyond this, skip them
                            current_section.indent_stack.truncate(idx);
                            idx
                        }
                    };
                    current_section.bullets.push((idx, bullet_line))
                } else {
                    let _ = writeln!(&mut additional_lines, "{line}");
                }
            } else {
                let _ = writeln!(&mut additional_lines, "{line}");
            }
        }

        self.flush(data, &mut current_section);

        if !additional_lines.is_empty() {
            self.additional.push((data.clone(), additional_lines));
        }
    }

    fn flush(&mut self, pr: &PrData, section: &mut Option<SectionState>) {
        if let Some(section) = section.take() {
            if section.entry.is_empty() {
                self.misformatted.push(pr.clone());
                return;
            }
            self.append_section(pr, section);
        }
    }

    fn append_section(&mut self, pr: &PrData, section: SectionState) {
        let entry = ChangelogEntry {
            number: pr.number,
            entry: section.entry,
            bullets: section.bullets,
        };
        self.sections.entry(section.krate).or_default().push(entry)
    }

    fn render(&self) {
        println!("\n\n# Crates\n=====================\n");
        // This is a silly but convenient way of doing indentation in format strings
        static INDENTATION: &str = "                                               ";

        for (header, entries) in &self.sections {
            println!("\n## {header}\n");
            for entry in entries {
                println!("- {} (unicode-org#{})", entry.entry, entry.number);
                for bullet in &entry.bullets {
                    println!(" {}- {}", &INDENTATION[..bullet.0], bullet.1);
                }
            }
        }

        println!("\n\n# PRs with additional notes\n=====================\n");

        for data in &self.additional {
            println!(
                "## {} (https://github.com/unicode-org/icu4x/pull/{})",
                data.0.title, data.0.number
            );
            println!("{}", data.1);
        }

        println!("\n\n# no changelog found\n=====================\n");
        for data in &self.no_changelog_found {
            println!(
                "## {} (https://github.com/unicode-org/icu4x/pull/{})",
                data.title, data.number
            );
            println!("{}", data.body);
        }
        println!("\n\n# Potentially misformatted (double check please!)\n=====================\n");
        for data in &self.misformatted {
            println!(
                "- {} (https://github.com/unicode-org/icu4x/pull/{})",
                data.title, data.number
            );
        }
        println!("\n\n# N/A\n=====================\n");
        for data in &self.n_a {
            println!(
                "- {} (https://github.com/unicode-org/icu4x/pull/{})",
                data.title, data.number
            );
        }
    }
}
