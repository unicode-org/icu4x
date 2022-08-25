// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use diplomat_core::*;
use rustdoc_types::{Crate, Item, ItemEnum};
use std::collections::{BTreeSet, HashSet};
use std::fmt;
use std::fs::File;
use std::path::PathBuf;
/// RustLink but without display information
#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord, Hash)]
struct RustLinkInfo {
    path: ast::Path,
    typ: ast::DocType,
}

impl fmt::Display for RustLinkInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}#{:?}", self.path, self.typ)
    }
}

fn main() {
    let doc_types = ["icu", "fixed_decimal"]
        .into_iter()
        .flat_map(collect_public_types)
        .map(|(path, typ)| RustLinkInfo {
            path: ast::Path {
                elements: path
                    .into_iter()
                    .map(|s| ast::Ident::try_from(s).expect("item path is valid"))
                    .collect(),
            },
            typ,
        })
        .filter(|rl| {
            ![
                ast::DocType::EnumVariant,
                ast::DocType::Mod,
                ast::DocType::Trait,
            ]
            .contains(&rl.typ)
        })
        .collect::<BTreeSet<_>>();

    let diplomat_crate = PathBuf::from(concat!(std::env!("CARGO_MANIFEST_DIR"), "/../src/lib.rs"));
    eprintln!("Loading Diplomat crate from {:?}", diplomat_crate);
    let diplomat_types =
        ast::File::from(&syn_inline_mod::parse_and_inline_modules(&diplomat_crate))
            .all_rust_links()
            .into_iter()
            .cloned()
            .map(|rl| RustLinkInfo {
                path: rl.path,
                typ: rl.typ,
            })
            .collect::<BTreeSet<_>>();

    doc_types
        .difference(&diplomat_types)
        .for_each(|item| println!("{item}"));
}

lazy_static::lazy_static! {
    static ref IGNORED_TRAITS: HashSet<&'static str> = [
        "Any",
        "AsULE",
        "Bake",
        "Borrow",
        "BorrowMut",
        "Clone",
        "Copy",
        "DataMarker",
        "Debug",
        "Default", // ???
        "Deserialize",
        "DeserializeOwned",
        "Display",
        "EncodeAsULE",
        "EncodeAsVarULE",
        "Eq",
        "ErasedDestructor",
        "Error",
        "From",
        "Hash",
        "Into",
        "IsCovariant",
        "Iterator", // ???
        "KeyedDataMarker",
        "Ord",
        "PartialEq",
        "PartialOrd",
        "RefUnwindSafe",
        "Send",
        "Separable",
        "Serialize",
        "StructuralEq",
        "StructuralPartialEq",
        "Sync",
        "ToOwned",
        "ToString", // ???
        "TryFrom", // ???
        "TryInto", // ???
        "ULE",
        "Unpin",
        "UnwindSafe",
        "VarULE",
        "Yokeable",
        "ZeroFrom",
    ].into_iter().collect();

    // Paths which are not checked for FFI coverage. Naming a type or module here
    // will include all type methods and module contents.
    static ref IGNORED_PATHS: HashSet<Vec<String>> = [
        // Stuff that could be exposed over FFI but is not currently planned
        // =========================
        // Largely for use by datetimeformat, not super public (#2421)
        "icu::calendar::week_of",


        // Individual calendars: Currently the main entry point is AnyCalendar
        "icu::calendar::buddhist",
        "icu::calendar::coptic",
        "icu::calendar::ethiopian",
        "icu::calendar::indian",
        "icu::calendar::japanese",
        "icu::calendar::julian",
        "icu::calendar::any_calendar::IntoAnyCalendar",

        // Arithmetic APIs are still experimental/hidden for 1.0
        "icu::calendar::DateDuration",
        "icu::calendar::DateDurationUnit",


        // Stuff that does not need to be exposed over FFI
        // Especially for stuff that are Rust specific like conversion traits
        // and markers and newtypes
        // =========================

        // "Internal" trait that should never be called directly
        "icu::calendar::Calendar",
        // Used for rust-specific type transforms
        "icu::calendar::AsCalendar",

        // FFI largely deals with primitives rather than Rust's nice wrapper types
        // (which are hard to do in a zero-cost way over FFI)
        "icu::calendar::types",

        // Provider modules
        // We could potentially expose them later, but it's hard to expose them
        // uniformly especially for complex types
        "icu::calendar::provider",
        "icu::datetime::provider",
    ].iter().map(|s| s.split("::").map(|x| x.to_string()).collect()).collect();
}

fn collect_public_types(krate: &str) -> impl Iterator<Item = (Vec<String>, ast::DocType)> {
    fn parse_doc(krate: &str) -> &Crate {
        lazy_static::lazy_static! {
            static ref CRATES: elsa::sync::FrozenMap<String, Box<Crate>> = elsa::sync::FrozenMap::new();
        }

        if CRATES.get(krate).is_none() {
            eprintln!("Parsing crate {krate}");
            let output = std::process::Command::new("cargo")
                .args(&[
                    "+nightly-2022-08-25",
                    "rustdoc",
                    "-p",
                    krate,
                    "--all-features",
                    "--",
                    "-Zunstable-options",
                    "--output-format",
                    "json",
                ])
                .output()
                .expect("failed to execute rustdoc");
            if !output.status.success() {
                panic!("Rustdoc build failed with {:?}", output);
            }
            let path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
                .join("../../../target/doc")
                .join(krate)
                .with_extension("json");
            eprintln!("Attempting to load {:?}", path);
            CRATES.insert(
                krate.to_string(),
                serde_json::from_reader(File::open(&path).unwrap()).unwrap(),
            );
        }
        CRATES.get(krate).unwrap()
    }

    enum In {
        Trait,
        Enum,
        Struct,
    }

    fn recurse(
        item: &Item,
        krate: &Crate,
        types: &mut HashSet<(Vec<String>, ast::DocType)>,
        mut path: Vec<String>,
        path_already_extended: bool,
        inside: Option<In>,
    ) {
        /// Helper function that ensures that IGNORED_PATHS
        /// is respected for every type inserted
        ///
        /// (We have a check at the beginning of recurse() but that won't catch leaf nodes)
        fn insert_ty(
            types: &mut HashSet<(Vec<String>, ast::DocType)>,
            path: Vec<String>,
            ty: ast::DocType,
        ) {
            if !IGNORED_PATHS.contains(&path) {
                types.insert((path, ty));
            }
        }
        if IGNORED_PATHS.contains(&path) {
            return;
        }
        match &item.inner {
            ItemEnum::Import(import) => {
                if !import.glob {
                    path.push(import.name.to_string());
                }

                if let Some(item) = &krate.index.get(import.id.as_ref().unwrap()) {
                    recurse(item, krate, types, path, true, None);
                } else if let Some(item) = &krate.paths.get(import.id.as_ref().unwrap()) {
                    // External crate. This is quite complicated and while it works, I'm not sure
                    // it's correct. This basically handles the case `pub use other_crate::module::Struct`,
                    // which means we have to parse `other_crate`, then look for `module`, then look
                    // for `Struct`. Now, `module` could actually be a reexport, which is why we have to
                    // check the `Import` case when traversing the path.
                    let external_crate = parse_doc(&krate.external_crates[&item.crate_id].name);
                    let mut item = &external_crate.index[&external_crate.root];
                    for segment in import.source.split("::").skip(1) {
                        match &item.inner {
                            ItemEnum::Module(inner) => {
                                item = inner
                                    .items
                                    .iter()
                                    .map(|id| &external_crate.index[id])
                                    .find(|item| match &item.inner {
                                        ItemEnum::Import(import) => {
                                            if import.name.as_str() == segment {
                                                path.pop();
                                                true
                                            } else {
                                                false
                                            }
                                        }
                                        _ => item.name.as_deref() == Some(segment),
                                    })
                                    .unwrap();
                            }
                            _ => unreachable!(),
                        }
                    }
                    recurse(item, external_crate, types, path, true, None);
                } else {
                    eprintln!("{:?} should be in either index or paths", path);
                }
            }
            _ => {
                if !path_already_extended {
                    path.push(item.name.as_ref().unwrap().to_string());
                }
                match &item.inner {
                    ItemEnum::Module(module) => {
                        for id in &module.items {
                            recurse(&krate.index[id], krate, types, path.clone(), false, None);
                        }
                        insert_ty(types, path, ast::DocType::Mod);
                    }
                    ItemEnum::Struct(structt) => {
                        for id in &structt.impls {
                            if let ItemEnum::Impl(inner) = &krate.index[id].inner {
                                if let Some(path) = &inner.trait_ {
                                    let name = &path.name;
                                    if IGNORED_TRAITS.contains(name.as_str()) {
                                        continue;
                                    }
                                }
                                for id in &inner.items {
                                    recurse(
                                        &krate.index[id],
                                        krate,
                                        types,
                                        path.clone(),
                                        false,
                                        Some(In::Struct),
                                    );
                                }
                                for name in &inner.provided_trait_methods {
                                    let mut path = path.clone();
                                    path.push(name.to_string());
                                    insert_ty(types, path, ast::DocType::FnInStruct);
                                }
                            }
                        }

                        insert_ty(types, path, ast::DocType::Struct);
                    }
                    ItemEnum::Enum(enumm) => {
                        for id in &enumm.variants {
                            recurse(&krate.index[id], krate, types, path.clone(), false, None);
                        }

                        for id in &enumm.impls {
                            if let ItemEnum::Impl(inner) = &krate.index[id].inner {
                                if let Some(path) = &inner.trait_ {
                                    let name = &path.name;
                                    if IGNORED_TRAITS.contains(name.as_str()) {
                                        continue;
                                    }
                                }
                                for id in &inner.items {
                                    recurse(
                                        &krate.index[id],
                                        krate,
                                        types,
                                        path.clone(),
                                        false,
                                        Some(In::Enum),
                                    );
                                }
                                for name in &inner.provided_trait_methods {
                                    let mut path = path.clone();
                                    path.push(name.to_string());
                                    insert_ty(types, path, ast::DocType::FnInEnum);
                                }
                            }
                        }

                        insert_ty(types, path, ast::DocType::Enum);
                    }
                    ItemEnum::Trait(inner) => {
                        for id in &inner.items {
                            recurse(
                                &krate.index[id],
                                krate,
                                types,
                                path.clone(),
                                false,
                                Some(In::Trait),
                            );
                        }
                        insert_ty(types, path, ast::DocType::Trait);
                    }
                    ItemEnum::Constant(_) => {
                        insert_ty(types, path, ast::DocType::Constant);
                    }
                    ItemEnum::Function(_) => {
                        insert_ty(types, path, ast::DocType::Fn);
                    }
                    ItemEnum::Macro(_) => {
                        insert_ty(types, path, ast::DocType::Macro);
                    }
                    ItemEnum::Typedef(_) => {
                        insert_ty(types, path, ast::DocType::Typedef);
                    }
                    ItemEnum::Method(_) => {
                        insert_ty(
                            types,
                            path,
                            match inside {
                                Some(In::Enum) => ast::DocType::FnInEnum,
                                Some(In::Trait) => ast::DocType::FnInTrait,
                                Some(In::Struct) => ast::DocType::FnInStruct,
                                _ => panic!("Method needs In"),
                            },
                        );
                    }
                    ItemEnum::Variant(_) => {
                        insert_ty(types, path, ast::DocType::EnumVariant);
                    }
                    ItemEnum::AssocConst { .. } => {
                        insert_ty(
                            types,
                            path,
                            match inside {
                                Some(In::Enum) => ast::DocType::AssociatedConstantInEnum,
                                Some(In::Trait) => ast::DocType::AssociatedConstantInTrait,
                                Some(In::Struct) => ast::DocType::AssociatedConstantInStruct,
                                _ => panic!("AssocConst needs In"),
                            },
                        );
                    }
                    ItemEnum::AssocType { .. } => {
                        insert_ty(
                            types,
                            path,
                            match inside {
                                Some(In::Enum) => ast::DocType::AssociatedTypeInEnum,
                                Some(In::Trait) => ast::DocType::AssociatedTypeInTrait,
                                Some(In::Struct) => ast::DocType::AssociatedTypeInStruct,
                                _ => panic!("AssocType needs In"),
                            },
                        );
                    }
                    _ => todo!("{:?}", item),
                }
            }
        }
    }

    let mut types = HashSet::new();
    let krate = parse_doc(krate);

    recurse(
        &krate.index[&krate.root],
        krate,
        &mut types,
        Vec::new(),
        false,
        None,
    );

    types.into_iter()
}
