use diplomat_core::*;
use rustdoc_types::{Crate, Item, ItemEnum, Type};
use std::collections::{BTreeSet, HashSet};
use std::fs::File;
use std::path::PathBuf;

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
}

fn collect_public_types(krate: &str) -> impl Iterator<Item = (Vec<String>, ast::DocType)> {
    fn parse_doc(krate: &str) -> &Crate {
        lazy_static::lazy_static! {
            static ref CRATES: elsa::sync::FrozenMap<String, Box<Crate>> = elsa::sync::FrozenMap::new();
        }

        if CRATES.get(krate).is_none() {
            eprintln!("Parsing crate {krate}");
            std::process::Command::new("cargo")
                .args(&[
                    "+nightly",
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
            CRATES.insert(
                krate.to_string(),
                serde_json::from_reader(
                    File::open(
                        &PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
                            .join("../../target/doc")
                            .join(krate)
                            .with_extension("json"),
                    )
                    .unwrap(),
                )
                .unwrap(),
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
                    recurse(item, &external_crate, types, path, true, None);
                } else {
                    unreachable!("id should be in either index or paths")
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
                        types.insert((path, ast::DocType::Mod));
                    }
                    ItemEnum::Struct(structt) => {
                        for id in &structt.impls {
                            if let ItemEnum::Impl(inner) = &krate.index[id].inner {
                                if let Some(Type::ResolvedPath { name, .. }) = &inner.trait_ {
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
                                    types.insert((path, ast::DocType::FnInStruct));
                                }
                            }
                        }

                        types.insert((path, ast::DocType::Struct));
                    }
                    ItemEnum::Enum(enumm) => {
                        for id in &enumm.variants {
                            recurse(&krate.index[id], krate, types, path.clone(), false, None);
                        }

                        for id in &enumm.impls {
                            if let ItemEnum::Impl(inner) = &krate.index[id].inner {
                                if let Some(Type::ResolvedPath { name, .. }) = &inner.trait_ {
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
                                    types.insert((path, ast::DocType::FnInEnum));
                                }
                            }
                        }

                        types.insert((path, ast::DocType::Enum));
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
                        types.insert((path, ast::DocType::Trait));
                    }
                    ItemEnum::Constant(_) => {
                        types.insert((path, ast::DocType::Constant));
                    }
                    ItemEnum::Function(_) => {
                        types.insert((path, ast::DocType::Fn));
                    }
                    ItemEnum::Macro(_) => {
                        types.insert((path, ast::DocType::Macro));
                    }
                    ItemEnum::Typedef(_) => {
                        types.insert((path, ast::DocType::Typedef));
                    }
                    ItemEnum::Method(_) => {
                        types.insert((
                            path,
                            match inside {
                                Some(In::Enum) => ast::DocType::FnInEnum,
                                Some(In::Trait) => ast::DocType::FnInTrait,
                                Some(In::Struct) => ast::DocType::FnInStruct,
                                _ => panic!("Method needs In"),
                            },
                        ));
                    }
                    ItemEnum::Variant(_) => {
                        types.insert((path, ast::DocType::EnumVariant));
                    }
                    ItemEnum::AssocConst { .. } => {
                        types.insert((
                            path,
                            match inside {
                                Some(In::Enum) => ast::DocType::AssociatedConstantInEnum,
                                Some(In::Trait) => ast::DocType::AssociatedConstantInTrait,
                                Some(In::Struct) => ast::DocType::AssociatedConstantInStruct,
                                _ => panic!("AssocConst needs In"),
                            },
                        ));
                    }
                    ItemEnum::AssocType { .. } => {
                        types.insert((
                            path,
                            match inside {
                                Some(In::Enum) => ast::DocType::AssociatedTypeInEnum,
                                Some(In::Trait) => ast::DocType::AssociatedTypeInTrait,
                                Some(In::Struct) => ast::DocType::AssociatedTypeInStruct,
                                _ => panic!("AssocType needs In"),
                            },
                        ));
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
        &krate,
        &mut types,
        Vec::new(),
        false,
        None,
    );

    types.into_iter()
}

fn main() {
    let doc_types = ["icu"]
        .into_iter()
        .flat_map(collect_public_types)
        .map(|(path, typ)| ast::RustLink {
            path: ast::Path {
                elements: path
                    .into_iter()
                    .map(|s| ast::Ident::try_from(s).expect("item path is valid"))
                    .collect(),
            },
            typ,
        })
        .filter(|rl| ![ast::DocType::EnumVariant, ast::DocType::Mod].contains(&rl.typ))
        .collect::<BTreeSet<_>>();

    let diplomat_types = ast::File::from(&syn_inline_mod::parse_and_inline_modules(
        &PathBuf::from(concat!(std::env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")),
    ))
    .all_rust_links()
    .into_iter()
    .cloned()
    .collect::<BTreeSet<_>>();

    doc_types
        .difference(&diplomat_types)
        .for_each(|item| println!("{item}"));
}
