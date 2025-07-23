// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! These arrays all list dependencies that are allowed in various contexts

/// Dependencies that are always allowed as runtime dependencies
/// In general it is fine to add new ICU4X components or utils here
/// For other crates, please get approval from @unicode-org/icu4x-owners
///
/// Keep in sync with Cargo.toml crates.io dependencies.
pub const BASIC_RUNTIME_DEPS: &[&str] = &[
    // ICU4X components
    "icu",
    "icu_calendar",
    "icu_casemap",
    "icu_collator",
    "icu_collections",
    "icu_datetime",
    "icu_decimal",
    "icu_list",
    "icu_locale",
    "icu_locale_core",
    "icu_normalizer",
    "icu_pattern",
    "icu_plurals",
    "icu_properties",
    "icu_segmenter",
    "icu_time",
    // ICU4X utils
    "calendrical_calculations",
    "fixed_decimal",
    "icu_provider",
    "litemap",
    "tinystr",
    "potential_utf",
    "writeable",
    "yoke",
    "zerofrom",
    "zerotrie",
    "zerovec",
    // 3P dependencies
    "core_maths",
    "either",
    "libm",
    "memchr",
    "regex-automata",
    "smallvec",
    "stable_deref_trait",
    "utf16_iter",
    "utf8_iter",
    "write16",
];

/// Dependencies that are always allowed as buildtime dependencies
/// In general it is fine to add new ICU4X components or utils here
/// For other crates, please get approval from @unicode-org/icu4x-owners
pub const BASIC_BUILD_DEPS: &[&str] = &[
    "displaydoc",
    "proc-macro2",
    "quote",
    "syn",
    "synstructure",
    "unicode-ident",
    "yoke-derive",
    "zerofrom-derive",
    "zerovec-derive",
];

/// Dependencies allowed when opting in to serialization
/// This should almost never change
///
/// Keep in sync with Cargo.toml crates.io dependencies.
pub const EXTRA_SERDE_DEPS: &[&str] = &["serde", "serde_derive"];

/// Dependencies allowed when opting in to compiled data
pub const EXTRA_DATA_DEPS: &[&str] = &[
    "icu_provider_baked",
    "icu_calendar_data",
    "icu_casemap_data",
    "icu_collator_data",
    "icu_datetime_data",
    "icu_decimal_data",
    "icu_list_data",
    "icu_locale_data",
    "icu_normalizer_data",
    "icu_plurals_data",
    "icu_properties_data",
    "icu_segmenter_data",
    "icu_time_data",
];

/// Dependencies allowed when opting in to experimental code
/// This will likely grow when we add experimental crates
///
/// Keep in sync with Cargo.toml crates.io dependencies.
pub const EXTRA_EXPERIMENTAL_DEPS: &[&str] = &[
    "icu_experimental",
    "icu_pattern",
    "num-bigint",
    "num-integer",
    "num-rational",
    "num-traits",
];

/// Dependencies allowed when opting in to compiled data
/// for experimental crates.
pub const EXTRA_EXPERIMENTAL_DATA_DEPS: &[&str] = &["icu_experimental_data"];

/// Dependencies allowed when opting in to LSTM segmenter
///
/// Keep in sync with Cargo.toml crates.io dependencies.
pub const EXTRA_LSTM_DEPS: &[&str] = &[];

/// Dependencies allowed when opting in to fixed_decimal's `ryu` backend
/// This should never change
///
/// Keep in sync with Cargo.toml crates.io dependencies.
pub const EXTRA_RYU_DEPS: &[&str] = &["ryu"];

/// Runtime dependencies allowed when building `icu_capi`
/// This shuld almost never change
///
/// Keep in sync with Cargo.toml crates.io dependencies.
pub const EXTRA_CAPI_DEPS: &[&str] = &[
    "diplomat-runtime",
    "icu_provider_adapters",
    "ixdtf",
    "unicode-bidi",
];

/// Build-time dependencies allowed when building `icu_capi`
/// This may change as Diplomat evolves, but care should be taken to keep this small
pub const EXTRA_CAPI_BUILD_DEPS: &[&str] = &["diplomat", "diplomat_core", "strck", "strck_ident"];

/// Dependencies allowed when opting in to blob providers on FFI
/// This shuld rarely change
///
/// Keep in sync with Cargo.toml crates.io dependencies.
pub const EXTRA_BLOB_DEPS: &[&str] = &["cobs", "icu_provider_blob", "postcard"];

/// Dependencies allowed when opting in to FS providers on FFI
/// This shuld rarely change
///
/// Keep in sync with Cargo.toml crates.io dependencies.
pub const EXTRA_FS_DEPS: &[&str] = &[
    "databake-derive",
    "databake",
    "erased-serde",
    "icu_provider_fs",
    "icu_provider_registry",
    "serde-json-core",
    "typeid",
];

/// Dependencies needed by datagen provider (not counting `log` and `zip` deps)
/// This might change semi frequently but we should try and keep this small.
pub const EXTRA_SOURCE_DEPS: &[&str] = &[
    "databake",
    "databake-derive",
    "elsa",
    "erased-serde",
    "equivalent",
    "filetime",
    "hashbrown",
    "icu_codepointtrie_builder",
    "icu_provider_adapters",
    "icu_provider_registry",
    "indexmap",
    "itertools",
    "itoa",
    "ixdtf",
    "libc",
    "matrixmultiply",
    "ndarray",
    "num-complex",
    "parse-zoneinfo",
    "rawpointer",
    "regex",
    "regex-syntax",
    "ryu",
    "serde-aux",
    "serde_json",
    "serde_spanned",
    "static_assertions",
    "tar",
    "toml_edit",
    "toml_datetime",
    "thiserror",
    "thiserror-impl",
    "typeid",
    "toml",
    "twox-hash",
    "winnow",
];

/// Dependencies needed by datagen (not counting `log` and `rayon` deps)
/// This might change semi frequently but we should try and keep this small.
pub const EXTRA_EXPORT_DEPS: &[&str] = &[
    "cobs",
    "databake",
    "databake-derive",
    "erased-serde",
    "postcard",
    "typeid",
];

/// Dependencies needed by the `log` crate
/// This should rarely change, and if it does consider toggling features until it doesn't
///
/// Keep in sync with Cargo.toml crates.io dependencies.
pub const EXTRA_LOGGING_DEPS: &[&str] = &["cfg-if", "log"];

/// Dependencies needed by the `zip` crate
/// This should rarely change, and if it does consider toggling features until it doesn't
pub const EXTRA_ZIP_DEPS: &[&str] = &[
    "adler2",
    "bumpalo",
    "byteorder",
    "crc32fast",
    "flate2",
    "lockfree-object-pool",
    "miniz_oxide",
    "once_cell",
    "ordered-float",
    "serde-spanned",
    "serde-value",
    "simd-adler32",
    "typed-arena",
    "zip",
    "zopfli",
];

/// Dependencies needed by the `rayon` crate
/// This should rarely change, and if it does consider toggling features until it doesn't
pub const EXTRA_RAYON_DEPS: &[&str] = &[
    "crossbeam-deque",
    "crossbeam-epoch",
    "crossbeam-utils",
    "rayon",
    "rayon-core",
];
