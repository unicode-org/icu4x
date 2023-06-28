// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! These arrays all list dependencies that are allowed in various contexts

/// Dependencies that are always allowed as runtime dependencies
/// In general it is fine to add new ICU4X components or utils here
/// For other crates, please get approval from @unicode-org/icu4x-owners
pub const BASIC_RUNTIME_DEPS: &[&str] = &[
    "either",
    "fixed_decimal",
    "icu",
    "icu_calendar",
    "icu_collator",
    "icu_collections",
    "icu_datetime",
    "icu_decimal",
    "icu_list",
    "icu_locid",
    "icu_locid_transform",
    "icu_normalizer",
    "icu_plurals",
    "icu_properties",
    "icu_provider",
    "icu_provider_adapters", // not included in icu, but needed generally
    "icu_segmenter",
    "icu_timezone",
    "libm",
    "litemap",
    "memchr",
    "regex-automata",
    "smallvec",
    "stable_deref_trait",
    "tinystr",
    "utf16_iter",
    "utf8_iter",
    "write16",
    "writeable",
    "yoke",
    "zerofrom",
    "zerovec",
];

/// Dependencies that are always allowed as buildtime dependencies
/// In general it is fine to add new ICU4X components or utils here
/// For other crates, please get approval from @unicode-org/icu4x-owners
pub const BASIC_BUILD_DEPS: &[&str] = &[
    "displaydoc",
    "icu_provider_macros",
    "proc-macro2",
    "quote",
    "syn",
    "synstructure",
    "unicode-ident",
    "unicode-xid",
    "yoke-derive",
    "zerofrom-derive",
    "zerovec-derive",
];

/// Dependencies allowed when opting in to serialization
/// This should almost never change
pub const EXTRA_SERDE_DEPS: &[&str] = &["deduplicating_array", "serde", "serde_derive"];

/// Dependencies allowed when opting in to experimental code
/// This will likely grow when we add experimental crates
pub const EXTRA_EXPERIMENTAL_DEPS: &[&str] = &[
    "icu_casemapping",
    "icu_displaynames",
    "icu_personnames",
    "icu_relativetime",
    "icu_compactdecimal",
];

/// Dependencies allowed when opting in to LSTM segmenter
pub const EXTRA_LSTM_DEPS: &[&str] = &["libm"];

/// Dependencies allowed when opting in to fixed_decimal's `ryu` backend
/// This should never change
pub const EXTRA_RYU_DEPS: &[&str] = &["ryu"];

/// Runtime dependencies allowed when building `icu_capi`
/// This shuld almost never change
pub const EXTRA_CAPI_DEPS: &[&str] = &["diplomat-runtime", "icu_capi", "unicode-bidi"];

/// Build-time dependencies allowed when building `icu_capi`
/// This may change as Diplomat evolves, but care should be taken to keep this small
pub const EXTRA_CAPI_BUILD_DEPS: &[&str] = &[
    "diplomat",
    "diplomat_core",
    "lazy_static",
    "strck",
    "strck_ident",
];

/// Dependencies allowed when opting in to blob providers on FFI
/// This shuld rarely change
pub const EXTRA_BLOB_DEPS: &[&str] = &["cobs", "icu_provider_blob", "postcard"];

/// Dependencies allowed when opting in to FS providers on FFI
/// This shuld rarely change
pub const EXTRA_FS_DEPS: &[&str] = &["icu_provider_fs", "serde-json-core"];

/// Dependencies allowed when opting in to test data on FFI
/// This shuld rarely change
pub const EXTRA_TEST_DEPS: &[&str] = &["icu_testdata"];

/// Dependencies needed by datagen (not counting `log`, `zip`, and `rayon` deps)
/// This might change semi frequently but we should try and keep this small.
pub const EXTRA_DATAGEN_DEPS: &[&str] = &[
    "bincode",
    "crlify",
    "databake",
    "databake-derive",
    "elsa",
    "erased-serde",
    "icu_codepointtrie_builder",
    "icu_datagen",
    "itertools",
    "itoa",
    "lazy_static",
    "matrixmultiply",
    "ndarray",
    "num-complex",
    "num-integer",
    "num-traits",
    "rawpointer",
    "regex-syntax",
    "ryu",
    "serde-aux",
    "serde_json",
    "toml",
];

/// Dependencies needed by the `log` crate
/// This should rarely change, and if it does consider toggling features until it doesn't
pub const EXTRA_LOGGING_DEPS: &[&str] = &["cfg-if", "log"];

/// Dependencies needed by the `zip` crate
/// This should rarely change, and if it does consider toggling features until it doesn't
pub const EXTRA_ZIP_DEPS: &[&str] = &[
    "adler",
    "byteorder",
    "cfg-if",
    "crc32fast",
    "crc32fast",
    "flate2",
    "miniz_oxide",
    "thiserror",
    "thiserror-impl",
    "zip",
];

/// Dependencies needed by the `rayon` crate
/// This should rarely change, and if it does consider toggling features until it doesn't
pub const EXTRA_RAYON_DEPS: &[&str] = &[
    "crossbeam-channel",
    "crossbeam-deque",
    "crossbeam-epoch",
    "crossbeam-utils",
    "libc",
    "memoffset",
    "num_cpus",
    "rayon",
    "rayon-core",
    "scopeguard",
];
