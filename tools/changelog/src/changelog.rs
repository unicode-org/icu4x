// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::args::MakeChangelog;
use crate::github::{GithubState, PrData};
use cargo_metadata::MetadataCommand;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;
use std::sync::LazyLock;

pub(crate) fn run(args: MakeChangelog) {
    let state = GithubState::load(&args.json);
    let crate_categories = load_crate_categories();
    let mut organized = OrganizedChangelog::new(crate_categories);
    for data in state.revs.values() {
        organized.add(data);
    }

    organized.render();
}

fn load_crate_categories() -> HashMap<String, Category> {
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to run cargo metadata");

    let mut map = HashMap::new();
    for package in metadata.workspace_packages() {
        let category = if package
            .manifest_path
            .components()
            .any(|c| c.as_str() == "components")
        {
            Category::Components
        } else if package
            .manifest_path
            .components()
            .any(|c| c.as_str() == "provider")
        {
            Category::Data
        } else if package
            .manifest_path
            .components()
            .any(|c| c.as_str() == "ffi")
        {
            Category::Ffi
        } else if package
            .manifest_path
            .components()
            .any(|c| c.as_str() == "utils" || c.as_str() == "tools")
        {
            Category::Utils
        } else {
            Category::Components
        };
        map.insert(package.name.clone(), category);
    }
    map
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Category {
    Components,
    Data,
    Ffi,
    Utils,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::Components => write!(f, "Components"),
            Category::Data => write!(f, "Data model and providers"),
            Category::Ffi => write!(f, "FFI"),
            Category::Utils => write!(f, "Utils"),
        }
    }
}

#[derive(Debug)]
struct OrganizedChangelog {
    crate_categories: HashMap<String, Category>,
    /// Category -> Option<Crate> -> Entries
    sections: BTreeMap<Category, BTreeMap<Option<String>, Vec<ChangelogEntry>>>,
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
    LazyLock::new(|| Regex::new("^(?<crate>`?\\S+`?:|`\\S+`)(?<entry>.*)$").unwrap());

#[derive(Clone, Debug)]
struct SectionState {
    category: Category,
    krate: Option<String>,
    entry: String,
    bullets: Vec<(usize, String)>,
    indent_stack: Vec<usize>,
}

impl OrganizedChangelog {
    fn new(crate_categories: HashMap<String, Category>) -> Self {
        Self {
            crate_categories,
            sections: BTreeMap::new(),
            additional: Vec::new(),
            n_a: Vec::new(),
            no_changelog_found: Vec::new(),
            misformatted: Vec::new(),
        }
    }

    fn get_category<'a>(&self, krate: &'a str) -> (Category, Option<&'a str>) {
        if krate == "General" || krate == "Components" || krate == "components" {
            return (Category::Components, None);
        }
        if krate == "Data" || krate == "data" || krate == "Data model and providers" {
            return (Category::Data, None);
        }
        if krate == "FFI" || krate == "ffi" {
            return (Category::Ffi, None);
        }
        if krate == "Utils" || krate == "utils" {
            return (Category::Utils, None);
        }

        if let Some(&cat) = self.crate_categories.get(krate) {
            return (cat, Some(krate));
        }

        if krate.contains('/') {
            let part_before = krate.split('/').next().unwrap();
            let (cat, _) = self.get_category(part_before);
            return (cat, Some(krate));
        }

        let mut sorted_crates: Vec<&String> = self.crate_categories.keys().collect();
        sorted_crates.sort_by_key(|b| std::cmp::Reverse(b.len()));

        for &c in &sorted_crates {
            if krate.starts_with(c) {
                return (self.crate_categories[c], Some(krate));
            }
        }

        if krate.contains("provider")
            || krate.contains("datagen")
            || krate.contains("source")
            || krate.contains("registry")
            || krate.contains("metadata")
        {
            return (Category::Data, Some(krate));
        }

        if krate.contains("capi") || krate.contains("harfbuzz") || krate.contains("diplomat") {
            return (Category::Ffi, Some(krate));
        }

        (Category::Components, Some(krate))
    }

    fn add(&mut self, data: &PrData) {
        if data.is_dependabot() {
            return;
        }

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
                let krate_str = header
                    .name("crate")
                    .unwrap()
                    .as_str()
                    .trim_matches(':')
                    .trim_matches('`')
                    .to_owned();

                let (category, krate) = self.get_category(&krate_str);

                current_section = Some(SectionState {
                    category,
                    krate: krate.map(String::from),
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
                            current_section.indent_stack.push(bullet_index);
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
            if additional_lines.trim().starts_with("N/A") {
                self.n_a.push(data.clone());
            } else {
                self.additional.push((data.clone(), additional_lines));
            }
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
        self.sections
            .entry(section.category)
            .or_default()
            .entry(section.krate)
            .or_default()
            .push(entry)
    }

    fn render(&self) {
        println!("\n\n# Crates\n=====================\n");

        for (category, krates) in &self.sections {
            println!("- {category}");
            for (krate, entries) in krates {
                match krate {
                    Some(krate) => {
                        println!("  - `{krate}`");
                    }
                    None => {
                        println!("  - General");
                    }
                }
                for entry in entries {
                    println!("    - {} (unicode-org#{})", entry.entry, entry.number);
                    for bullet in &entry.bullets {
                        let indent = 6 + bullet.0 * 2;
                        println!("{:indent$}- {}", "", bullet.1, indent = indent);
                    }
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
