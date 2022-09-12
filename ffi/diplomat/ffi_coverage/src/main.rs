// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use diplomat_core::*;
use rustdoc_types::{Crate, Item, ItemEnum};
use std::collections::{BTreeSet, HashMap, HashSet};
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
    let doc_types = ["icu", "fixed_decimal", "icu_provider_adapters"]
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
        // Rust and Rust ecosystem types
        "Any",
        "AsMut",
        "AsRef",
        "Bake",
        "Borrow",
        "BorrowMut",
        "Clone",
        "Copy",
        "Debug",
        "Default", // ???
        "Deserialize",
        "DeserializeOwned",
        "Display",
        "Eq",
        "ErasedDestructor",
        "Error",
        "From",
        "Hash",
        "Into",
        "Iterator", // ???
        "Ord",
        "Provider", // new error handling stuff
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
        "Unpin",
        "UnwindSafe",

        // yoke/zerovec/etc internals
        "ULE",
        "AsULE",
        "VarULE",
        "Yokeable",
        "ZeroFrom",
        "ZeroMapKV",
        "EncodeAsULE",
        "EncodeAsVarULE",
        "IsCovariant",

        // provider stuff not relevant to FFI
        "DataMarker",
        "KeyedDataMarker",
        "AsDowncastingAnyProvider",
        "AsDeserializingBufferProvider",
        "AsDynamicDataProviderAnyMarkerWrap",
        "IterableDynamicDataProvider",
        "IterableDataProvider",
        "DataConverter",
        "Filterable",

        // The four main data provider traits should be covered if the enum or struct
        // implementing them is covered
        "DataProvider",
        "DynamicDataProvider",
        "BufferProvider",
        "AnyProvider",

        // internal trait , all methods replicated on Date
        "Calendar",
        // Rust-specific conversion trait
        "AsCalendar",
        "IntoAnyCalendar",
    ].into_iter().collect();

    static ref IGNORED_ASSOCIATED_ITEMS: HashMap<&'static str, &'static [&'static str]> = [
        ("Writeable", &["writeable_length_hint", "write_to_parts", "write_to_string"][..]),
        ("FromStr", &["Err"][..]),
    ].into_iter().collect();

    // Ignore if this is a substring of any path
    // keep this small
    static ref IGNORED_SUBSTRINGS: &'static [&'static str] = &[
        // _unstable constructors cover these
        "with_any_provider",
        "with_buffer_provider",
    ];
    // Paths which are not checked for FFI coverage. Naming a type or module here
    // will include all type methods and module contents.
    static ref IGNORED_PATHS: HashSet<Vec<String>> = [
        // Stuff that we DO plan to expose for 1.0 but we're keeping in the ignorelist
        // because we plan to solve it all at once.
        // This section should go away before 1.0
        // =========================

        // currently empty

        // Stuff that could be exposed over FFI but is not currently planned (for 1.0)
        //
        // Post 1.0 we should go through this and plan them, filing followups
        // for ones we do plan and adding links here
        // https://github.com/unicode-org/icu4x/issues/2492
        // =========================

        // Largely for use by datetimeformat, not generally useful
        "icu::calendar::AnyCalendar::convert_any_date",
        "icu::calendar::AnyCalendar::convert_any_datetime",

        // Punted post 1.0: not strongly needed yet and don't want to lock in a solution
        // Potential solutions:
        // - borrow and clone (cheap as long it's not json)
        // - introduce a DTFBorrowed type in Rust and FFI (bunch of work, annoying)
        // - introduce a DateDataBundle and TimeDataBundle struct to FFI that contains
        //   basically just DateFormat or TimeFormat but it is explicitly an Option that
        //   can be destructively passed to these constructors via &mut self. All future
        //   specialized constructors show up on this type instead.
        "icu::datetime::DateTimeFormatter::try_from_date_and_time",
        "icu::datetime::TypedDateTimeFormatter::try_from_date_and_time",

        // experimental
        "icu::datetime::DateTimeFormatter::resolve_components",
        "icu::datetime::TypedDateTimeFormatter::resolve_components",

        // Individual calendars: Currently the main entry point is AnyCalendar
        "icu::calendar::buddhist",
        "icu::calendar::coptic",
        "icu::calendar::ethiopian",
        "icu::calendar::indian",
        "icu::calendar::japanese",
        "icu::calendar::julian",
        "icu::calendar::any_calendar::IntoAnyCalendar",
        "icu::calendar::Date::new_gregorian_date",
        "icu::calendar::Date::new_buddhist_date",
        "icu::calendar::Date::new_coptic_date",
        "icu::calendar::Date::new_ethiopian_date",
        "icu::calendar::Date::new_indian_date",
        "icu::calendar::Date::new_japanese_date",
        "icu::calendar::Date::new_japanese_extended_date",
        "icu::calendar::Date::new_julian_date",
        "icu::calendar::DateTime::new_gregorian_datetime",
        "icu::calendar::DateTime::new_buddhist_datetime",
        "icu::calendar::DateTime::new_coptic_datetime",
        "icu::calendar::DateTime::new_ethiopian_datetime",
        "icu::calendar::DateTime::new_indian_datetime",
        "icu::calendar::DateTime::new_japanese_datetime",
        "icu::calendar::DateTime::new_japanese_extended_datetime",
        "icu::calendar::DateTime::new_julian_datetime",

        // Arithmetic APIs are still experimental/hidden for 1.0
        "icu::calendar::DateDuration",
        "icu::calendar::DateDurationUnit",

        // mostly used for provider, may in the future be exposed for options
        "icu::datetime::fields",
        // experimental
        "icu::datetime::options::components",
        "icu::datetime::options::preferences",
        "icu::datetime::DateTimeFormatter::try_new_experimental_unstable",
        "icu::datetime::TypedDateTimeFormatter::try_new_experimental_unstable",
        "icu::datetime::TypedZonedDateTimeFormatter::try_new_experimental_unstable",
        "icu::datetime::ZonedDateTimeFormatter::try_new_experimental_unstable",

        // Not necessary for now
        "icu::calendar::Date::day_of_year_info",


        // Formatting wrappers, may be supported in the future
        "icu::datetime::FormattedTimeZone",
        "icu::datetime::FormattedDateTime",
        "icu::datetime::FormattedZonedDateTime",
        "icu::decimal::FormattedFixedDecimal",
        "icu::decimal::format::FormattedFixedDecimal",

        // Rust-specific power user API for rules ASTS and such
        // could be exposed in the future but it's complicated
        "icu::plurals::rules",

        // May be exposed when we have associated constants over FFI
        "icu::properties::BidiClass",
        "icu::properties::CanonicalCombiningClass",
        "icu::properties::EastAsianWidth",
        "icu::properties::GeneralCategory",
        "icu::properties::GeneralCategoryGroup",
        "icu::properties::GraphemeClusterBreak",
        "icu::properties::LineBreak",
        "icu::properties::Script",
        "icu::properties::SentenceBreak",
        "icu::properties::WordBreak",

        // Experimental
        "icu::properties::maps::load_canonical_combining_class",

        // Not planned for 1.0
        "icu::properties::maps::CodePointMapDataBorrowed::iter_ranges",
        "icu::properties::sets::CodePointSetDataBorrowed::iter_ranges",
        "icu::properties::maps::CodePointMapData::as_code_point_trie",
        "icu::properties::maps::CodePointMapData::from_code_point_trie",
        "icu::properties::sets::CodePointSetData::as_code_point_inversion_list",
        "icu::properties::sets::CodePointSetData::from_code_point_inversion_list",
        "icu::properties::sets::CodePointSetData::to_code_point_inversion_list",
        "icu::collections::codepointinvlist",
        "icu::collections::codepointtrie",

        // Not planned until someone needs them
        "icu::locid::extensions",
        "icu::locid::subtags",
        "icu::locid::LanguageIdentifier",

        // experimental
        "icu::normalizer::ComposingNormalizer::try_new_uts46_without_ignored_and_disallowed_unstable",

        // can't be exposed till Diplomat has Write16
        "icu::normalizer::ComposingNormalizer::normalize_utf16",
        "icu::normalizer::ComposingNormalizer::normalize_utf16_to",
        "icu::normalizer::ComposingNormalizer::is_normalized_utf16",
        "icu::normalizer::DecomposingNormalizer::normalize_utf16",
        "icu::normalizer::DecomposingNormalizer::normalize_utf16_to",
        "icu::normalizer::DecomposingNormalizer::is_normalized_utf16",

        // Can't be exposed till diplomat has input iterators, as well as
        // safety for borrowing input iterators into return types
        "icu::normalizer::ComposingNormalizer::normalize_iter",
        "icu::normalizer::DecomposingNormalizer::normalize_iter",
        "icu::normalizer::Composition",
        "icu::normalizer::Decomposition",

        // Need to think about how to expose DataErrorKind for this to work
        "icu_provider_adapters::empty::EmptyDataProvider::new_with_error_kind",

        // We should add this once we have a better story for FFI custom data structs
        "icu_provider_adapters::any_payload::AnyPayloadProvider",

        // We don't expose data keys directly over FFI, but when we do, we should add this
        "icu_provider_adapters::fallback::LocaleFallbacker::for_key",

        // Don't want parts for 1.0
        "icu::list::parts",
        // Formatting wrappers, may be supported in the future
        "icu::list::FormattedList",

        // Experimental
        "icu::casemapping",

        // Stuff that does not need to be exposed over FFI
        // Especially for stuff that are Rust specific like conversion traits
        // and markers and newtypes
        // =========================

        // Provider modules
        // We could potentially expose them later, but it's hard to expose them
        // uniformly especially for complex types
        "icu::calendar::provider",
        "icu::datetime::provider",
        "icu::locid_transform::provider",
        "icu::plurals::provider",
        "icu::properties::provider",
        "icu::segmenter::provider",
        "icu::normalizer::provider",
        "icu::list::provider",
        "icu::timezone::provider",
        "icu::collator::provider",
        "icu::decimal::provider",
        "icu_provider_adapters::fallback::provider",

        // Reexports (tool doesn't currently handle these)
        "icu::calendar::any_calendar::AnyCalendar",
        "icu::calendar::any_calendar::AnyCalendarKind",
        "icu::datetime::time_zone::TimeZoneFormatter",
        "icu::datetime::options::DateTimeFormatterOptions",
        "icu::decimal::options::GroupingStrategy",
        "icu::decimal::options::FixedDecimalFormatterOptions",

        // "Internal" trait that should never be called directly
        "icu::calendar::Calendar",
        // Rust-specific calendar wrapper stuff
        "icu::calendar::AsCalendar",
        "icu::datetime::CldrCalendar",
        "icu::calendar::Ref",
        "icu::calendar::Date::wrap_calendar_in_rc",
        "icu::calendar::Date::wrap_calendar_in_arc",
        "icu::calendar::DateTime::wrap_calendar_in_rc",
        "icu::calendar::DateTime::wrap_calendar_in_arc",

        // Individual markerlike calendar types and inner types
        // inner types are only public for associated type reasons, and the markerlike
        // calendar types exist to implement the trait
        "icu::calendar::Date::from_raw",
        "icu::calendar::Date::inner",
        "icu::calendar::Iso",
        "icu::calendar::iso::Iso",
        "icu::calendar::iso::IsoDateInner",
        "icu::calendar::Gregorian",
        "icu::calendar::gregorian::Gregorian",
        "icu::calendar::gregorian::GregorianDateInner",
        "icu::calendar::any_calendar::AnyDateInner",

        // Rusty input trait
        "icu::datetime::input",

        // Options bags which are expanded in FFI to regular functions
        "icu::datetime::DateTimeFormatterOptions",
        "icu::datetime::time_zone::TimeZoneFormatterOptions",
        "icu::datetime::options::length::Bag",

        // FFI largely deals with primitives rather than Rust's nice wrapper types
        // (which are hard to do in a zero-cost way over FFI)
        "icu::calendar::types",

        // Convenience iterator for Rust
        "icu::plurals::PluralCategory::all",
        // associated type
        "icu::plurals::PluralOperands::Err",

        // Properties Rust internals
        "icu::properties::maps::CodePointMapData::as_borrowed",
        "icu::properties::maps::CodePointMapData::from_data",
        "icu::properties::maps::CodePointMapData::to_code_point_trie",
        "icu::properties::maps::CodePointMapData::try_into_converted",
        "icu::properties::sets::CodePointSetData::as_borrowed",
        "icu::properties::sets::CodePointSetData::from_data",
        "icu::properties::sets::CodePointSetDataBorrowed::contains_u32",

        // locid macros
        "icu::locid::langid",
        "icu::locid::locale",
        "icu::locid::extensions_other_key",
        "icu::locid::extensions_private_key",
        "icu::locid::extensions_transform_key",
        "icu::locid::extensions_unicode_attribute",
        "icu::locid::extensions_unicode_key",
        "icu::locid::extensions_unicode_value",
        "icu::locid::subtags_language",
        "icu::locid::subtags_region",
        "icu::locid::subtags_script",
        "icu::locid::subtags_variant",
        // assoc type
        "icu::locale::Locale::Err",

        // Segmenter types and type aliases that are constructed via methods. They don't need FFI.
        "icu::segmenter::GraphemeClusterBreakIteratorLatin1",
        "icu::segmenter::GraphemeClusterBreakIteratorUtf16",
        "icu::segmenter::GraphemeClusterBreakIteratorUtf8",
        "icu::segmenter::GraphemeClusterBreakIteratorPotentiallyIllFormedUtf8",
        "icu::segmenter::LineBreakIterator",
        "icu::segmenter::LineBreakIteratorLatin1",
        "icu::segmenter::LineBreakIteratorUtf16",
        "icu::segmenter::LineBreakIteratorUtf8",
        "icu::segmenter::LineBreakIteratorPotentiallyIllFormedUtf8",
        "icu::segmenter::RuleBreakIterator",
        "icu::segmenter::SentenceBreakIteratorLatin1",
        "icu::segmenter::SentenceBreakIteratorUtf16",
        "icu::segmenter::SentenceBreakIteratorUtf8",
        "icu::segmenter::SentenceBreakIteratorPotentiallyIllFormedUtf8",
        "icu::segmenter::WordBreakIteratorLatin1",
        "icu::segmenter::WordBreakIteratorUtf16",
        "icu::segmenter::WordBreakIteratorUtf8",
        "icu::segmenter::WordBreakIteratorPotentiallyIllFormedUtf8",

        // Some of the provider adapter types are Rust-specific and not relevant to FFI
        "icu_provider_adapters::either::EitherProvider",

        // Decompositions of providers is tricky to do over FFI and the use cases are unclear.
        "icu_provider_adapters::fallback::LocaleFallbackProvider::inner",
        "icu_provider_adapters::fallback::LocaleFallbackProvider::into_inner",

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

    #[derive(Debug)]
    enum In<'a> {
        Trait,
        // The Option<String> is for the trait name of an impl
        Enum(Option<&'a str>),
        Struct(Option<&'a str>),
    }

    fn recurse(
        item: &Item,
        krate: &Crate,
        types: &mut HashSet<(Vec<String>, ast::DocType)>,
        mut path: Vec<String>,
        path_already_extended: bool,
        inside: Option<In>,
    ) {
        fn ignored(path: &Vec<String>) -> bool {
            IGNORED_PATHS.contains(path)
                || path
                    .last()
                    .map(|l| IGNORED_SUBSTRINGS.iter().any(|i| l.contains(i)))
                    .unwrap_or(false)
        }
        /// Helper function that ensures that ignored()
        /// is respected for every type inserted
        ///
        /// (We have a check at the beginning of recurse() but that won't catch leaf nodes)
        fn insert_ty(
            types: &mut HashSet<(Vec<String>, ast::DocType)>,
            path: Vec<String>,
            ty: ast::DocType,
        ) {
            if !ignored(&path) {
                types.insert((path, ty));
            }
        }

        fn check_ignored_assoc_item(name: &str, trait_path: Option<&str>) -> bool {
            if let Some(tr) = trait_path {
                if let Some(ignored) = IGNORED_ASSOCIATED_ITEMS.get(tr) {
                    if ignored.contains(&name) {
                        return true;
                    }
                }
            }
            false
        }

        if ignored(&path) {
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
                let item_name = item.name.as_ref().unwrap();
                if !path_already_extended {
                    path.push(item_name.to_string());
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
                                let mut trait_name = None;
                                if let Some(path) = &inner.trait_ {
                                    let name = &path.name;
                                    if IGNORED_TRAITS.contains(name.as_str()) {
                                        continue;
                                    }
                                    trait_name = Some(&*path.name);
                                }
                                for id in &inner.items {
                                    recurse(
                                        &krate.index[id],
                                        krate,
                                        types,
                                        path.clone(),
                                        false,
                                        Some(In::Struct(trait_name)),
                                    );
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
                                let mut trait_name = None;
                                if let Some(path) = &inner.trait_ {
                                    let name = &path.name;
                                    if IGNORED_TRAITS.contains(name.as_str()) {
                                        continue;
                                    }
                                    trait_name = Some(&*path.name);
                                }
                                for id in &inner.items {
                                    recurse(
                                        &krate.index[id],
                                        krate,
                                        types,
                                        path.clone(),
                                        false,
                                        Some(In::Enum(trait_name)),
                                    );
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
                        let doc_type = match inside {
                            Some(In::Enum(tr)) | Some(In::Struct(tr))
                                if check_ignored_assoc_item(item_name, tr) =>
                            {
                                return
                            }
                            Some(In::Enum(_)) => ast::DocType::FnInEnum,
                            Some(In::Trait) => ast::DocType::FnInTrait,
                            Some(In::Struct(_)) => ast::DocType::FnInStruct,
                            _ => panic!("Method needs In"),
                        };
                        insert_ty(types, path, doc_type);
                    }
                    ItemEnum::Variant(_) => {
                        insert_ty(types, path, ast::DocType::EnumVariant);
                    }
                    ItemEnum::AssocConst { .. } => {
                        let doc_type = match inside {
                            Some(In::Enum(tr)) | Some(In::Struct(tr))
                                if check_ignored_assoc_item(item_name, tr) =>
                            {
                                return
                            }
                            Some(In::Enum(_)) => ast::DocType::AssociatedConstantInEnum,
                            Some(In::Trait) => ast::DocType::AssociatedConstantInTrait,
                            Some(In::Struct(_)) => ast::DocType::AssociatedConstantInStruct,
                            _ => panic!("AssocConst needs In"),
                        };
                        insert_ty(types, path, doc_type);
                    }
                    ItemEnum::AssocType { .. } => {
                        let doc_type = match inside {
                            Some(In::Enum(tr)) | Some(In::Struct(tr))
                                if check_ignored_assoc_item(item_name, tr) =>
                            {
                                return
                            }
                            Some(In::Enum(_)) => ast::DocType::AssociatedTypeInEnum,
                            Some(In::Trait) => ast::DocType::AssociatedTypeInTrait,
                            Some(In::Struct(_)) => ast::DocType::AssociatedTypeInStruct,
                            _ => panic!("AssocType needs In"),
                        };
                        insert_ty(types, path, doc_type);
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
