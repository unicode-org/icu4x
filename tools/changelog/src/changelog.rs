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
static HTML_COMMENT: LazyLock<Regex> = LazyLock::new(|| Regex::new("(?s)<!--.*?-->").unwrap());
/// Crate headers of the forms `` `crate`: entry ``, `` `crate` entry ``, `[crate] entry`, `crate: entry`
static SECTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        "^(?:",
        "`(?<c1>[^`\\s]+)`:?(?:\\s+(?<e1>.*))?",
        "|\\[(?<c2>[^\\]\\s]+)\\]:?(?:\\s+(?<e2>.*))?",
        "|(?<c3>[^\\s:`\\[\\]]+):(?:\\s+(?<e3>.*))?",
        ")$"
    ))
    .unwrap()
});
/// A bare word (with optional colon), only accepted if it's a known crate or category
static BARE_SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new("^(?<crate>[\\w/-]+):?$").unwrap());

/// A line (or bullet point) of a changelog, with its nested lines.
#[derive(Debug)]
struct Node {
    text: String,
    is_bullet: bool,
    children: Vec<Node>,
}

impl Node {
    /// Parses the changelog text into a tree based on indentation.
    fn parse_tree(text: &str) -> Vec<Node> {
        let text = HTML_COMMENT.replace_all(text, "");

        // Stack of (indentation key, path of child indices from the root).
        // Bullets are considered nested under non-bullet lines with the same indentation.
        let mut roots: Vec<Node> = Vec::new();
        let mut stack: Vec<((usize, bool), usize)> = Vec::new();

        fn get<'a>(roots: &'a mut [Node], stack: &[((usize, bool), usize)]) -> &'a mut Node {
            let mut node = &mut roots[stack[0].1];
            for &(_, idx) in &stack[1..] {
                node = &mut node.children[idx];
            }
            node
        }

        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let indent = line.len() - line.trim_start().len();
            let bullet_text = ["- ", "* ", "+ "]
                .iter()
                .find_map(|m| trimmed.strip_prefix(m))
                .or_else(|| ["-", "*", "+"].contains(&trimmed).then_some(""));

            let Some(bullet_text) = bullet_text else {
                if indent > 0 && !stack.is_empty() {
                    // Continuation of a wrapped line
                    let node = get(&mut roots, &stack);
                    node.text.push(' ');
                    node.text.push_str(trimmed);
                    continue;
                }
                stack.clear();
                roots.push(Node {
                    text: trimmed.to_owned(),
                    is_bullet: false,
                    children: Vec::new(),
                });
                stack.push(((indent, false), roots.len() - 1));
                continue;
            };

            let key = (indent, true);
            while stack.last().is_some_and(|&(k, _)| k >= key) {
                stack.pop();
            }
            let node = Node {
                text: bullet_text.trim().to_owned(),
                is_bullet: true,
                children: Vec::new(),
            };
            let siblings = if stack.is_empty() {
                &mut roots
            } else {
                &mut get(&mut roots, &stack).children
            };
            siblings.push(node);
            let idx = siblings.len() - 1;
            stack.push((key, idx));
        }
        roots
    }

    /// Flattens descendants into (depth, text) bullets.
    fn flatten_children(&self, depth: usize, out: &mut Vec<(usize, String)>) {
        for child in &self.children {
            out.push((depth, child.text.clone()));
            child.flatten_children(depth + 1, out);
        }
    }

    fn render(&self, depth: usize, out: &mut String) {
        let bullet = if self.is_bullet { "- " } else { "" };
        let _ = writeln!(
            out,
            "{:indent$}{bullet}{}",
            "",
            self.text,
            indent = depth * 2
        );
        for child in &self.children {
            child.render(depth + 1, out);
        }
    }
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

        let mut found_anything = false;
        let mut additional_lines = String::new();
        for node in Node::parse_tree(changelog_data) {
            found_anything = true;
            if let Some((krate_str, entry)) = self.parse_section_header(&node.text) {
                let (category, krate) = self.get_category(&krate_str);
                let krate = krate.map(String::from);
                self.add_section(data, &node, category, krate, entry);
            } else {
                node.render(0, &mut additional_lines);
            }
        }

        if !found_anything {
            self.no_changelog_found.push(data.clone());
        } else if !additional_lines.is_empty() {
            if additional_lines.trim().starts_with("N/A") {
                self.n_a.push(data.clone());
            } else {
                self.additional.push((data.clone(), additional_lines));
            }
        }
    }

    /// Parses a line as a section header, returning the crate (or category) and the entry text.
    fn parse_section_header(&self, text: &str) -> Option<(String, String)> {
        if let Some(caps) = SECTION.captures(text) {
            let krate = ["c1", "c2", "c3"]
                .iter()
                .find_map(|n| caps.name(n))
                .unwrap()
                .as_str();
            let entry = ["e1", "e2", "e3"]
                .iter()
                .find_map(|n| caps.name(n))
                .map(|e| e.as_str().trim())
                .unwrap_or_default();
            return Some((krate.to_owned(), entry.to_owned()));
        }
        if let Some(caps) = BARE_SECTION.captures(text) {
            let krate = caps.name("crate").unwrap().as_str();
            let is_category = matches!(
                krate,
                "General"
                    | "Components"
                    | "components"
                    | "Data"
                    | "data"
                    | "FFI"
                    | "ffi"
                    | "Utils"
                    | "utils"
            );
            if is_category || self.crate_categories.contains_key(krate) {
                return Some((krate.to_owned(), String::new()));
            }
        }
        None
    }

    fn add_section(
        &mut self,
        pr: &PrData,
        node: &Node,
        category: Category,
        krate: Option<String>,
        entry: String,
    ) {
        if !entry.is_empty() {
            let mut bullets = Vec::new();
            node.flatten_children(0, &mut bullets);
            self.append_entry(pr, category, krate, entry, bullets);
            return;
        }

        if node.children.is_empty() {
            self.misformatted.push(pr.clone());
            return;
        }

        // `icu_foo:` followed by a list of entries, or `General:` followed by a list of crates/entries.
        for child in &node.children {
            if krate.is_none()
                && let Some((child_krate_str, child_entry)) = self.parse_section_header(&child.text)
            {
                let (mut child_category, child_krate) = self.get_category(&child_krate_str);
                if child_krate.is_some() && !self.crate_categories.contains_key(&child_krate_str) {
                    // Unknown crate, trust the enclosing category
                    child_category = category;
                }
                let child_krate = child_krate.map(String::from);
                self.add_section(pr, child, child_category, child_krate, child_entry);
                continue;
            }
            let mut bullets = Vec::new();
            child.flatten_children(0, &mut bullets);
            self.append_entry(pr, category, krate.clone(), child.text.clone(), bullets);
        }
    }

    fn append_entry(
        &mut self,
        pr: &PrData,
        category: Category,
        krate: Option<String>,
        entry: String,
        bullets: Vec<(usize, String)>,
    ) {
        let entry = ChangelogEntry {
            number: pr.number,
            entry,
            bullets,
        };
        self.sections
            .entry(category)
            .or_default()
            .entry(krate)
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
