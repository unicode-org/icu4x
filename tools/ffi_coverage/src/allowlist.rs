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
        "Default", // We expose this when we see fit
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
        "PartialEq",
        "PartialOrd",
        "Provider", // From stdlib error infrastructure
        "RefUnwindSafe",
        "Send",
        "Separable",
        "Serialize",
        "StructuralEq",
        "StructuralPartialEq",
        "Sync",
        "ToOwned",
        "ToString", // We expose this when we see fit
        "TryFrom", // We expose this when we see fit
        "TryInto", // We expose this when we see fit
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
        "DynamicDataMarker",
        "DataMarker",
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

        // internal trait, all methods replicated on Date
        "Calendar",
        // Rust-specific conversion trait
        "AsCalendar",
        "IntoAnyCalendar",
        "NeoGetField",
    ].into_iter().collect();

    pub static ref IGNORED_ASSOCIATED_ITEMS: HashMap<&'static str, &'static [&'static str]> = [
        ("Writeable", &["writeable_length_hint", "write_to_parts", "write_to_string"][..]),
        ("FromStr", &["Err"][..]),
    ].into_iter().collect();

    // Ignore if this is a substring of any path
    // keep this small
    pub static ref IGNORED_SUBSTRINGS: &'static [&'static str] = &[
        // compiled data constructors cover these
        "_with_any_provider",
        // TODO-2.0 remove this
        "_with_buffer_provider",
        "_unstable",
    ];
    // Paths which are not checked for FFI coverage. Naming a type or module here
    // will include all type methods and module contents.
    pub static ref IGNORED_PATHS: HashSet<Vec<String>> = [
        // Stuff that could be exposed over FFI but is not currently planned (for 2.0)
        //
        // Post 2.0 we should go through this and plan them, filing followups
        // for ones we do plan and adding links here
        // https://github.com/unicode-org/icu4x/issues/2492
        // =========================

        // Largely for use by datetimeformat, not generally useful
        "icu::calendar::AnyCalendar::convert_any_date",
        "icu::calendar::AnyCalendar::convert_any_datetime",

        // Individual calendars: Currently the main entry point is AnyCalendar
        // We have chosen to not do individual calendars (except Iso) over FFI
        // since Diplomat can't do generics. We also support Gregorian *formatter*
        // but we don't need a separate Gregorian Date.
        "icu::calendar::buddhist",
        "icu::calendar::chinese",
        "icu::calendar::coptic",
        "icu::calendar::dangi",
        "icu::calendar::dangi",
        "icu::calendar::ethiopian",
        "icu::calendar::hebrew",
        "icu::calendar::indian",
        "icu::calendar::islamic",
        "icu::calendar::japanese",
        "icu::calendar::julian",
        "icu::calendar::persian",
        "icu::calendar::roc",
        "icu::calendar::any_calendar::IntoAnyCalendar",
        "icu::calendar::Date::try_new_buddhist_date",
        "icu::calendar::Date::try_new_chinese_date_with_calendar",
        "icu::calendar::Date::try_new_coptic_date",
        "icu::calendar::Date::try_new_dangi_date",
        "icu::calendar::Date::try_new_dangi_date_with_calendar",
        "icu::calendar::Date::try_new_ethiopian_date",
        "icu::calendar::Date::try_new_gregorian_date",
        "icu::calendar::Date::try_new_hebrew_date",
        "icu::calendar::Date::try_new_hebrew_date_with_calendar",
        "icu::calendar::Date::try_new_indian_date",
        "icu::calendar::Date::try_new_islamic_civil_date_with_calendar",
        "icu::calendar::Date::try_new_islamic_tabular_date_with_calendar",
        "icu::calendar::Date::try_new_japanese_date",
        "icu::calendar::Date::try_new_japanese_extended_date",
        "icu::calendar::Date::try_new_julian_date",
        "icu::calendar::Date::try_new_observational_islamic_date",
        "icu::calendar::Date::try_new_persian_date",
        "icu::calendar::Date::try_new_roc_date",
        "icu::calendar::Date::try_new_ummalqura_date",
        "icu::calendar::DateTime::try_new_buddhist_datetime",
        "icu::calendar::DateTime::try_new_chinese_datetime_with_calendar",
        "icu::calendar::DateTime::try_new_coptic_datetime",
        "icu::calendar::DateTime::try_new_dangi_datetime",
        "icu::calendar::DateTime::try_new_dangi_datetime_with_calendar",
        "icu::calendar::DateTime::try_new_ethiopian_datetime",
        "icu::calendar::DateTime::try_new_gregorian_datetime",
        "icu::calendar::DateTime::try_new_hebrew_datetime",
        "icu::calendar::DateTime::try_new_hebrew_datetime_with_calendar",
        "icu::calendar::DateTime::try_new_indian_datetime",
        "icu::calendar::DateTime::try_new_islamic_civil_datetime_with_calendar",
        "icu::calendar::DateTime::try_new_islamic_tabular_datetime_with_calendar",
        "icu::calendar::DateTime::try_new_japanese_datetime",
        "icu::calendar::DateTime::try_new_japanese_extended_datetime",
        "icu::calendar::DateTime::try_new_julian_datetime",
        "icu::calendar::DateTime::try_new_observational_islamic_datetime",
        "icu::calendar::DateTime::try_new_persian_datetime",
        "icu::calendar::DateTime::try_new_roc_datetime",
        "icu::calendar::DateTime::try_new_ummalqura_datetime",

        // Calendar structs mostly for internal use but which might expose
        // useful information to clients.
        "icu::calendar::types::FormattableMonth",
        "icu::calendar::types::FormattableYear",
        "icu::calendar::types::DayOfYearInfo",

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

        // Experimental API mostly used for provider, components bags, and patterns,
        // may in the future be exposed for options
        "icu::datetime::fields",

        // experimental
        "icu::datetime::neo",
        "icu::datetime::neo_marker",
        "icu::datetime::neo_pattern",
        "icu::datetime::neo_skeleton",
        "icu::datetime::options::components",
        "icu::datetime::options::preferences",
        "icu::datetime::DateTimeFormatter::try_new_experimental",
        "icu::datetime::DateTimeWriteError",
        "icu::datetime::LoadError",
        "icu::datetime::SingleLoadError",
        "icu::datetime::FormattedDateTimePattern",
        "icu::datetime::TypedDateTimeNames",
        "icu::datetime::TypedDateTimeFormatter::try_new_experimental",
        "icu::datetime::TypedZonedDateTimeFormatter::try_new_experimental",
        "icu::datetime::ZonedDateTimeFormatter::try_new_experimental",

        // experimental
        "icu::experimental",

        // Experimental and unused decimal types
        "fixed_decimal::CompactDecimal",
        "fixed_decimal::FixedInteger",
        "fixed_decimal::ScientificDecimal",

        // Don't want parts for 2.0, would need to introduce diplomat writeable with parts
        "icu::list::parts",

        // Not planned until someone needs them
        "icu::locale::extensions",
        "icu::locale::subtags",

        // TODO-2.0: decide later when we have figured out prefs/ctors and have APIs using this
        "icu::locale::LanguageIdentifier",

        // Doesn't make sense to expose through `icu_normalizer`
        "icu::normalizer::uts46::Uts46Mapper",

        // Do not want for 2.0: we need DiplomatWriteable16
        "icu::normalizer::ComposingNormalizer::normalize_utf16",
        "icu::normalizer::ComposingNormalizer::normalize_utf16_to",
        "icu::normalizer::ComposingNormalizer::is_normalized_utf16",
        "icu::normalizer::DecomposingNormalizer::normalize_utf16",
        "icu::normalizer::DecomposingNormalizer::normalize_utf16_to",
        "icu::normalizer::DecomposingNormalizer::is_normalized_utf16",

        // Do not want for 2.0:
        // Can't be exposed till diplomat has input iterators, as well as
        // safety for borrowing input iterators into return types
        "icu::normalizer::ComposingNormalizer::normalize_iter",
        "icu::normalizer::DecomposingNormalizer::normalize_iter",
        "icu::normalizer::Composition",
        "icu::normalizer::Decomposition",

        // experimental
        "icu::plurals::rules",

        // Experimental
        "icu::plurals::PluralRulesWithRanges",
        "icu::plurals::PluralRulesWithRanges::categories",
        "icu::plurals::PluralRulesWithRanges::category_for",
        "icu::plurals::PluralRulesWithRanges::category_for_range",
        "icu::plurals::PluralRulesWithRanges::resolve_range",
        "icu::plurals::PluralRulesWithRanges::try_new",
        "icu::plurals::PluralRulesWithRanges::try_new_cardinal",
        "icu::plurals::PluralRulesWithRanges::try_new_ordinal",

        // Not planned for 2.0
        // We aren't exposing these collections directly, we instead expose them in a domain specific
        // way like CodePointSetDataBuilder. We may eventually add these as utilities for users.
        "icu::collections",
        "icu::properties::maps::CodePointMapData::as_code_point_trie",
        "icu::properties::maps::CodePointMapData::from_code_point_trie",
        "icu::properties::maps::CodePointMapData::to_code_point_trie",
        "icu::properties::maps::CodePointMapDataBorrowed::iter_ranges",
        "icu::properties::sets::UnicodeSetData::as_code_point_inversion_list_string_list",
        "icu::properties::sets::UnicodeSetData::from_code_point_inversion_list_string_list",
        "icu::properties::sets::UnicodeSetData::to_code_point_inversion_list_string_list",

        // We do not plan to have FFI for this in 2.0
        "icu_provider_adapters::empty::EmptyDataProvider",

        // We should add this once we have a better story for FFI custom data structs
        // and callbacks
        "icu_provider_adapters::any_payload::AnyPayloadProvider",

        // Not planned for 2.0
        // FilterDataProvider::with_filter needs callbacks.
        "icu_provider_adapters::filter::FilterDataProvider",

        // Not planned for 2.0
        // ForkByErrorProvider is the abstract forking provider; we expose the concrete
        // fork by locale/key ones. Could be added if we have callbacks.
        "icu_provider_adapters::fork::ForkByErrorProvider",
        "icu_provider_adapters::fork::predicates::ForkByErrorPredicate",

        // Not planned for 2.0 but would be nice to return 'static refs
        // with Diplomat support.
        // Borrowed <-> owned converters
        "icu::locale::fallback::LocaleFallbacker::as_borrowed",
        "icu::locale::fallback::LocaleFallbackerBorrowed::static_to_owned",
        "icu::properties::bidi_data::BidiAuxiliaryProperties::as_borrowed",
        "icu::properties::bidi_data::BidiAuxiliaryPropertiesBorrowed::static_to_owned",
        "icu::properties::maps::CodePointMapData::as_borrowed",
        "icu::properties::maps::CodePointMapDataBorrowed::static_to_owned",
        "icu::properties::names::PropertyEnumToValueNameLinearMapper::as_borrowed",
        "icu::properties::names::PropertyEnumToValueNameLinearMapperBorrowed::static_to_owned",
        "icu::properties::names::PropertyEnumToValueNameLinearTiny4Mapper::as_borrowed",
        "icu::properties::names::PropertyEnumToValueNameLinearTiny4MapperBorrowed::static_to_owned",
        "icu::properties::names::PropertyEnumToValueNameSparseMapper::as_borrowed",
        "icu::properties::names::PropertyEnumToValueNameSparseMapperBorrowed::static_to_owned",
        "icu::properties::names::PropertyValueNameToEnumMapper::as_borrowed",
        "icu::properties::names::PropertyValueNameToEnumMapperBorrowed::static_to_owned",
        "icu::properties::script::ScriptWithExtensions::as_borrowed",
        "icu::properties::script::ScriptWithExtensionsBorrowed::static_to_owned",
        "icu::properties::sets::CodePointSetData::as_borrowed",
        "icu::properties::sets::CodePointSetDataBorrowed::static_to_owned",
        "icu::properties::sets::UnicodeSetData::as_borrowed",
        "icu::properties::sets::UnicodeSetDataBorrowed::static_to_owned",

        // Stuff that does not need to be exposed over FFI
        // Especially for stuff that are Rust specific like conversion traits
        // and markers and newtypes
        // =========================

        // Datagen
        "icu::markers_for_bin",

        // Provider modules
        // We could potentially expose them later, but it's hard to expose them
        // uniformly especially for complex types
        "icu::calendar::provider",
        "icu::casemap::provider",
        "icu::collator::provider",
        "icu::datetime::provider",
        "icu::decimal::provider",
        "icu::list::provider",
        "icu::locale::provider",
        "icu::normalizer::provider",
        "icu::plurals::provider",
        "icu::properties::provider",
        "icu::segmenter::provider",
        "icu::timezone::provider",
        "icu::transliterate::provider",

        // ULE types that are not in provider modules
        "icu::collections::codepointinvlist::CodePointInversionListULE",
        "icu::plurals::PluralCategoryULE",

        // Reexported
        "icu::calendar::any_calendar::AnyCalendar",
        "icu::calendar::any_calendar::AnyCalendarKind",
        "icu::casemap::titlecase::TitlecaseMapper",
        "icu::calendar::types::Time",


        // TODO-2.0 these errors will have changed
        "fixed_decimal::Error",
        "icu::calendar::Error",
        "icu::collator::Error",
        "icu::collections::codepointinvlist::Error",
        "icu::compactdecimal::Error",
        "icu::datetime::Error",
        "icu::decimal::Error",
        "icu::list::Error",
        "icu::locale::Error",
        "icu::locale::Error",
        "icu::normalizer::Error",
        "icu::plurals::Error",
        "icu::properties::Error",
        "icu::relativetime::Error",
        "icu::segmenter::Error",
        "icu::timezone::Error",
        "icu::transliterator::Error",

        // "Internal" trait that should never be called directly
        "icu::calendar::Calendar",

        // Rust-specific calendar wrapper stuff
        "icu::calendar::AsCalendar",
        "icu::calendar::Ref",
        "icu::datetime::CldrCalendar",
        // TODO-2.0: needs investigation
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
        "icu::datetime::NeverCalendar",

        // Options bags which are expanded in FFI to regular functions
        // TODO-2.0: investigate flattening on the rust side too
        "icu::datetime::DateTimeFormatterOptions",
        "icu::datetime::options::DateTimeFormatterOptions",
        "icu::datetime::options::length::Bag",
        "icu::decimal::options::FixedDecimalFormatterOptions",

        // Constructing an error is not useful over FFI because it gets turned into
        // a flat ICU4XError anyway
        "icu::calendar::CalendarError::unknown_any_calendar_kind",

        // FFI largely deals with primitives rather than Rust's nice wrapper types
        // (which are hard to do in a zero-cost way over FFI)
        "icu::calendar::types::MonthCode",
        "icu::calendar::types::DayOfMonth",
        "icu::calendar::types::WeekOfMonth",
        "icu::calendar::types::WeekOfYear",
        "icu::calendar::types::DayOfWeekInMonth",
        "icu::calendar::types::IsoHour",
        "icu::calendar::types::IsoMinute",
        "icu::calendar::types::IsoSecond",
        "icu::calendar::types::NanoSecond",
        "icu::calendar::types::IsoWeekday",
        "icu::calendar::types::Era",

        // Rusty input trait
        "icu::datetime::input",

        // Convenience iterator for Rust. Useful but would require
        // allocations over FFI, so not worth it.
        "icu::plurals::PluralCategory::all",

        // locale_core comparison iteration
        "icu::locale::Locale::strict_cmp_iter",
        "icu::locale::SubtagOrderingResult",

        // Some of the provider adapter types are Rust-specific and not relevant to FFI
        "icu_provider_adapters::either::EitherProvider",

        // Decompositions of providers is tricky to do over FFI and the use cases are unclear.
        "icu_provider_adapters::fallback::LocaleFallbackProvider::inner",
        "icu_provider_adapters::fallback::LocaleFallbackProvider::into_inner",
        "icu_provider_adapters::fallback::LocaleFallbackProvider::inner_mut",

        // The polymorphic ICU4XDataProvider type makes the MultiFork providers less relevant.
        "icu_provider_adapters::fork::MultiForkByErrorProvider",
        "icu_provider_adapters::fork::MultiForkByMarkerProvider",

        // Specialized constructor for separately constructed instances
        "icu::timezone::TimeZoneIdMapperWithFastCanonicalization::try_new_with_mapper",

        // macros
        "icu::locale::langid",
        "icu::locale::locale",
        "icu::locale::extensions::other::subtag",
        "icu::locale::extensions::private::subtag",
        "icu::locale::extensions::transform::key",
        "icu::locale::extensions::unicode::attribute",
        "icu::locale::extensions::unicode::key",
        "icu::locale::extensions::unicode::value",
        "icu::locale::subtags::language",
        "icu::locale::subtags::region",
        "icu::locale::subtags::script",
        "icu::locale::subtags::variant",
        "icu_provider_adapters::make_forking_provider",

        // assoc types
        "icu::locale::Locale::Err",
        "icu::plurals::PluralOperands::Err",

    ].iter().map(|s| s.split("::").map(str::to_owned).collect()).collect();
}
