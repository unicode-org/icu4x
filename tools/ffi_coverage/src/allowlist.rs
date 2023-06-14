// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This file contains the allowlist for the ffi_coverage test
//!
//! Most new entries will go into IGNORED_PATHS

use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    pub static ref IGNORED_TRAITS: HashSet<&'static str> = [
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
        "IntoIterator", // skip IntoIterator but not Iterator
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
        "ForkByErrorPredicate",

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

    pub static ref IGNORED_ASSOCIATED_ITEMS: HashMap<&'static str, &'static [&'static str]> = [
        ("Writeable", &["writeable_length_hint", "write_to_parts", "write_to_string"][..]),
        ("FromStr", &["Err"][..]),
    ].into_iter().collect();

    // Ignore if this is a substring of any path
    // keep this small
    pub static ref IGNORED_SUBSTRINGS: &'static [&'static str] = &[
        // _unstable constructors cover these
        "with_any_provider",
        "with_buffer_provider",
    ];
    // Paths which are not checked for FFI coverage. Naming a type or module here
    // will include all type methods and module contents.
    pub static ref IGNORED_PATHS: HashSet<Vec<String>> = [
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
        "icu::calendar::Date::try_new_gregorian_date",
        "icu::calendar::Date::try_new_buddhist_date",
        "icu::calendar::Date::try_new_coptic_date",
        "icu::calendar::Date::try_new_ethiopian_date",
        "icu::calendar::Date::try_new_indian_date",
        "icu::calendar::Date::try_new_japanese_date",
        "icu::calendar::Date::try_new_japanese_extended_date",
        "icu::calendar::Date::try_new_julian_date",
        "icu::calendar::DateTime::try_new_gregorian_datetime",
        "icu::calendar::DateTime::try_new_buddhist_datetime",
        "icu::calendar::DateTime::try_new_coptic_datetime",
        "icu::calendar::DateTime::try_new_ethiopian_datetime",
        "icu::calendar::DateTime::try_new_indian_datetime",
        "icu::calendar::DateTime::try_new_japanese_datetime",
        "icu::calendar::DateTime::try_new_japanese_extended_datetime",
        "icu::calendar::DateTime::try_new_julian_datetime",

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

        // The FFI constructor takes a single option instead of a struct
        "icu::decimal::options::FixedDecimalFormatterOptions",

        // Experimental and unused decimal types
        "fixed_decimal::CompactDecimal",
        "fixed_decimal::FixedInteger",
        "fixed_decimal::ScientificDecimal",

        // Rust-specific power user API for rules ASTS and such
        // could be exposed in the future but it's complicated
        "icu::plurals::rules",

        // Pulls in libstd, which we'd rather not do
        "icu::plurals::PluralOperands::n",

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
        "icu::properties::maps::CodePointMapData::as_code_point_trie",
        "icu::properties::maps::CodePointMapData::from_code_point_trie",
        "icu::properties::maps::CodePointMapData::to_code_point_trie",
        "icu::properties::maps::CodePointMapDataBorrowed::iter_ranges",
        "icu::properties::sets::CodePointSetData::as_code_point_inversion_list",
        "icu::properties::sets::CodePointSetData::from_code_point_inversion_list",
        "icu::properties::sets::CodePointSetData::to_code_point_inversion_list",
        "icu::properties::sets::UnicodeSetData::as_code_point_inversion_list_string_list",
        "icu::properties::sets::UnicodeSetData::from_code_point_inversion_list_string_list",
        "icu::properties::sets::UnicodeSetData::to_code_point_inversion_list_string_list",
        "icu::properties::script::ScriptWithExtensionsBorrowed::get_script_extensions_set", // returns an inversion list
        "icu::collections::codepointinvlist",
        "icu::collections::codepointinvliststringlist",
        "icu::collections::codepointtrie",
        "icu::collections::char16trie",

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
        "icu::locid_transform::fallback::LocaleFallbackConfig::from_key",

        // On RequestFilterDataProvider, filter_by_langid needs callbacks, and
        // filter_by_langid_allowlist_strict needs input iterators.
        // require_langid is not very useful by itself.
        "icu_provider_adapters::filter::Filterable",
        "icu_provider_adapters::filter::RequestFilterDataProvider",

        // ForkByErrorProvider has only one useful constructor, new_with_predicate,
        // which needs callback support.
        "icu_provider_adapters::fork::ForkByErrorProvider",
        "icu_provider_adapters::fork::predicates::ForkByErrorPredicate",

        // Don't want parts for 1.0
        "icu::list::parts",
        // Formatting wrappers, may be supported in the future
        "icu::list::FormattedList",

        // Experimental
        "icu::casemapping",
        "icu::compactdecimal",
        "icu::relativetime",
        "icu::displaynames",

        // Stuff that does not need to be exposed over FFI
        // Especially for stuff that are Rust specific like conversion traits
        // and markers and newtypes
        // =========================

        // Provider modules
        // We could potentially expose them later, but it's hard to expose them
        // uniformly especially for complex types
        "icu::calendar::provider",
        "icu::collator::provider",
        "icu::datetime::provider",
        "icu::decimal::provider",
        "icu::list::provider",
        "icu::locid_transform::provider",
        "icu::normalizer::provider",
        "icu::plurals::provider",
        "icu::properties::provider",
        "icu::segmenter::provider",
        "icu::timezone::provider",

        // Reexports (tool doesn't currently handle these)
        "icu::calendar::any_calendar::AnyCalendar",
        "icu::calendar::any_calendar::AnyCalendarKind",
        "icu::datetime::options::DateTimeFormatterOptions",

        // Re-exports of errors
        "icu::calendar::Error",
        "icu::compactdecimal::Error",
        "icu::datetime::Error",
        "icu::decimal::Error",
        "icu::list::Error",
        "icu::locid::Error",
        "icu::locid_transform::Error",
        "icu::normalizer::Error",
        "icu::plurals::Error",
        "icu::properties::Error",
        "icu::relativetime::Error",
        "icu::segmenter::Error",
        "icu::timezone::Error",
        "icu::collator::Error",

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

        // Constructing an error is not useful over FFI because it gets turned into
        // a flat ICU4XError anyway
        "icu::calendar::CalendarError::unknown_any_calendar_kind",

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

        // locid macros
        "icu::locid::langid",
        "icu::locid::locale",
        "icu::locid::extensions_other_subtag",
        "icu::locid::extensions_private_subtag",
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

        // locid comparison iteration
        "icu::locid::Locale::strict_cmp_iter",
        "icu::locid::SubtagOrderingResult",

        // The only UTF-8 currently allowed over FFI is potentially-ill-formed.
        "icu::segmenter::GraphemeClusterBreakIteratorUtf8",
        "icu::segmenter::LineBreakIteratorUtf8",
        "icu::segmenter::SentenceBreakIteratorUtf8",
        "icu::segmenter::WordBreakIteratorUtf8",

        // Some of the provider adapter types are Rust-specific and not relevant to FFI
        "icu_provider_adapters::either::EitherProvider",

        // Decompositions of providers is tricky to do over FFI and the use cases are unclear.
        "icu_provider_adapters::fallback::LocaleFallbackProvider::inner",
        "icu_provider_adapters::fallback::LocaleFallbackProvider::into_inner",
        "icu_provider_adapters::fallback::LocaleFallbackProvider::inner_mut",

        // The polymorphic ICU4XDataProvider type makes the MultiFork providers less relevant.
        "icu_provider_adapters::fork::MultiForkByErrorProvider",
        "icu_provider_adapters::fork::MultiForkByKeyProvider",

        // No macros in FFI
        "icu_provider_adapters::make_forking_provider",

    ].iter().map(|s| s.split("::").map(str::to_owned).collect()).collect();
}
