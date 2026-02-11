// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
// #![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        clippy::trivially_copy_pass_by_ref,
        // missing_debug_implementations,
    )
)]
// #![warn(missing_docs)]

//! # host_info
//!
//! `host_info` is a library providing functionality to retrieve regional preferences
//! from host environments - primarily the operating system the program is running in.
//!
//! The library is designed to bind the different host environment preference architectures
//! to the [`icu`] model.
//!
//! # Example
//!
//! ```ignore
//! use icu_host_info::icu_host_info;
//! use icu::calendar::Date;
//! use icu::datetime::{fieldsets, DateTimeFormatter};
//!
//! let date = Date::try_new_gregorian(2025, 10, 10)
//!     .expect("Failed to create date");
//!
//! // requires feature `datetime`
//! let prefs = icu_host_info::datetime_preferences()
//!     .expect("Failed to retrieve host info");
//!
//! let dtf = DateTimeFormatter::try_new(prefs, fieldsets::YMD::long())
//!     .expect("Failed to create datetime formatter.");
//!
//! let formatted_dt = dtf.format(&date);
//!
//! assert_eq!(formatted_dt.to_string(), "October 10, 2025");
//! ```
//!
//! # Feature Matrix
//!
//! The library intends to provide means to retrieve regional preferences
//! to [`icu`] preferences with a focus on Unicode Extensions, but allow for
//! propagation of preferences offered by the host environments which may
//! not have a representation in Unicode Extensions (for example: date format pattern).
//!
//! Legend:
//! - ✅ = OS + `host_info` support
//! - 🚧 = OS supports, `host_info` doesn't
//! - ❌ = OS doesn't support
//!
//! | Feature             | Android | iOS | Linux <sup>(1)</sup> | macOS | Windows |
//! |---------------------| :-----: | :-: | :------------------: | :---: | :-----: |
//! | Requested Locales   |   ✅    | ✅  | ✅                  |   ✅  |    ✅   |
//! | Calendar            |   🚧    | 🚧  | 🚧                  |   ✅  |    ✅   |
//! | Region              |   🚧    | 🚧  | 🚧                  |   ✅  |    ✅   |
//! | Hour cycle          |   🚧    | 🚧  | ✅                  |   ✅  |    🚧   |
//! | Measurement System  |   🚧    | 🚧  | 🚧                  |   ✅  |    🚧   |
//! | Measurement Override|   🚧    | 🚧  | 🚧                  |   ✅  |    🚧   |
//! | First Day of week   |   🚧    | 🚧  | 🚧                  |   ✅  |    ✅   |
//! | Collation           |   🚧    | 🚧  | 🚧                  |   ✅  |    ❌    |
//! | Date format         |   🚧    | 🚧  | 🚧                  |   🚧  |    🚧   |
//! | Number format       |   🚧    | 🚧  | 🚧                  |   🚧  |    🚧   |
//!
//! <sup>(1)</sup> In the case of Linux, different desktop environments such as Gnome and KDE are supported together.
//!
//! # Integrating preferences into ICU formatters
//!
//! The library provides three ways of injecting retrieved values into formatters:
//!
//! ## 1. Preference Bag
//!
//! For most common components, such as [`DateTimeFormatter`], the library exposes
//! a direct getter that retrieves a [`Preferences`] struct for that component.
//! This getter is located behind a flag to allow for control over which dependencies are being
//! pulled.
//!
//! ### Example
//!
//! ```ignore
//! use icu_host_info::icu_host_info;
//! use icu::datetime::{fieldsets, DateTimeFormatter};
//!
//! // requires feature `datetime`
//! let prefs = icu_host_info::datetime_preferences()
//!     .expect("Failed to retrieve host info");
//!
//! let dtf = DateTimeFormatter::try_new(prefs, fieldsets::YMD::long())
//!     .expect("Failed to create datetime formatter.");
//! ```
//!
//! ## 2. Locale
//!
//! For all components that `icu_host_info` does not have special preference getter for,
//! and for cases where the user prefers to avoid pulling extra dependencies at the cost
//! of narrowing down the retrieved values to just ones encoded in Unicode Extensions,
//! the library provides an ergonomic getter:
//!
//! ### Example
//!
//! ```
//! use icu::{
//!     datetime::{fieldsets, DateTimeFormatter},
//!     locale::Locale,
//! };
//!
//! let mut locale = icu_host_info::requested_locales()
//!     .expect("Failed to retrieve locales")
//!     .first()
//!     .cloned()
//!     .unwrap_or(Locale::UNKNOWN);
//!
//! locale.extensions.unicode = icu_host_info::unicode_extensions()
//!     .expect("Failed to retrieve host info");
//!
//! let dtf = DateTimeFormatter::try_new(locale.into(), fieldsets::YMD::long())
//!     .expect("Failed to create datetime formatter.");
//! ```
//!
//! Notice that the regional preferences encoded in Unicode Extensions
//! are retrieved separately from the list of requested locales.
//! There are two reasons for this design:
//! - The user has to decide whether the regional preferences apply onto all locales, or just the first one
//! - The locale negotiation may result in a different locale being selected.
//!
//! ## 3. Individual Preferences
//!
//! For each preference the library also attempts to provide a direct getter
//! allowing the user to retrieve just that preference and use it as they see fit.
//!
//! ### Example
//!
//! ```
//! use icu::locale::preferences::extensions::unicode::keywords::HourCycle;
//!
//! let mut calendar: Option<HourCycle> = icu_host_info::hour_cycle()
//!     .expect("Failed to retrieve hour_cycle preference");
//! ```
//!
//! # Locale Negotiation
//!
//! Locale Negotiation is an upcoming feature in ICU4X which will enable the system integrating ICU to
//! perform a negotiation between requested locales, and locales for which the data is available in the system.
//! The output of `icu_host_info` will be utilized in that negotiation allowing the deployment to 1) select
//! the most appropriate locales for the given user and target modality, 2) apply regional preferences onto that
//! locale.
//!
//! The need to allow `icu_host_info` to be pluggable into locale negotiation and multi source merging (see next section)
//! guided many design choices in this library. This section will be extended once locale negotiation is implemented.
//!
//! # Multi Source Merging
//!
//! In simple systems the user will most often use ICU to format
//! some information in a selected locale, and use this library to augment
//! the formatting with regional preferences set by the user in the host environment.
//!
//! In more complex systems, the user may also want to introduce a second source of regional preferences
//! and mix the values set in the host environment with those set in the program itself.
//!
//! For example, a web browser may offer some regional preferences set in the browser
//! itself, or even set separate for some contexts of the browser.
//!
//! In those cases, the deployment requires merging of the preferences.
//! ICU exposes an `extend` method on both [`Preferences`]  and [`Unicode`] extensions struct.
//!
//! This allows the system to retrieve Preferences or [`Unicode`], and the application's
//! equivalent, and merge them.
//!
//! ## `Preferences` Example
//!
//! ```ignore
//! use icu_host_info::icu_host_info;
//! use icu::datetime::{fieldsets, DateTimeFormatter};
//!
//! let app_prefs = app.datetime_preferences();
//!
//! // requires feature `datetime`
//! let mut combined_prefs = icu_host_info::datetime_preferences()
//!     .expect("Failed to retrieve host info");
//!
//! combined_prefs.extend(app_prefs);
//!
//! let dtf = DateTimeFormatter::try_new(combined_prefs, fieldsets::YMD::long())
//!     .expect("Failed to create datetime formatter.");
//! ```
//!
//! ## `Unicode` Extensions Example
//!
//! ```ignore
//! use icu_host_info::icu_host_info;
//! use icu::{
//!     datetime::{fieldsets, DateTimeFormatter},
//!     locale::locale,
//! };
//!
//! let mut locale = locale!("fr-CA");
//!
//! let app_ue = app.unicode_extensions();
//!
//! let mut combined_ue = icu_host_info::unicode_extensions()
//!     .expect("Failed to retrieve host info");
//!
//! combined_ue.extend(app_ue);
//!
//! locale.extensions.unicode = combined_ue;
//!
//! let dtf = DateTimeFormatter::try_new(locale.into(), fieldsets::YMD::long())
//!     .expect("Failed to create datetime formatter.");
//! ```
//!
//! # Design Decisions
//!
//! The library operates on a boundary of diverse set of host
//! environments and uniform ICU design derived from Unicode LDML.
//! It requires a number of design tradeoffs that had to be made in
//! order to achieve the uniformity and scale over time as the host
//! platforms design evolves.
//!
//! ## Host Environment
//!
//! The library is designed to handle retrieval of data from the direct host
//! environment. This usually means an operating system, but it can mean a
//! virtual environment, sandbox or runtime.
//! In such a case it is the responsibility of the execution logic
//! setting up such environment to ensure propagation of customer preferences.
//!
//! ## Lossy Results
//!
//! The library makes best-effort to retrieve the values
//! that can be directly used in ICU. As the operating systems,
//! runtimes and ICU evolve, there's always a risk of a mismatch.
//! This library makes a design decision to be lossy-by-default.
//!
//! Any value that cannot be directly mapped onto a valid value is ignored
//! and indistinguishable in the ergonomic API from a missing value.
//!
//! Similarly, the API does not distinguish between missing binding logic and unknown value.
//! The assumption is that users of this library are aiming to respect user choices
//! encoded in host environment regional preferences, but are not in a position
//! to act differently on a failed attempt to retrieve them from a missing attempt.
//! Therefore errors in this library are very rare and only related to catastrophic
//! cases like memory corruption or OS API errors propagation.
//!
//! ## Normalized vs Raw values
//!
//! The library provides methods that return normalized
//! values, often directly taken from `icu::locale_core::preferences`.
//! Per-host backends provide additional trait implementation that returns
//! raw values, allowing the user to handle or introspect those values manually.
//! When using `icu_host_info`, the library performs best-effort to normalize and parse
//! those raw values into canonical Unicode ICU representation, often discarding
//! unknown values and values that fail to parse.
//!
//! Those raw backends are not exposed in the main documentation.
//!
//! ### Example
//!
//! ```ignore
//! use icu_host_info::backends::{
//!     RawHostInfoBackend,
//!     macos::MacOSHostInfoBackend,
//! };
//!
//! let raw_cal: Option<String> = MacOSHostInfoBackend::raw_calendar()
//!     .expect("Failed to retrieve raw calendar");
//! ```
//!
//! ## Minimize defaults
//!
//! The library attempts to use host APIs in a way that allows distinguishing between
//! preference values that represent defaults for a given locale, from ones manually set
//! by the user.
//! In some cases, the host API does not allow for distinguishing of that, which may result
//! in overly expressive locales such as `en-US-ca-gregory` (`gregory` being already a default calendar for en-US).
//!
//! This, like other aspects of the library, operates on best-effort basis and may be further improved in the future
//! releases as better bindings become available.
//!
//! ### Host API Design Guidance
//!
//! A note for host API designers - it is useful for foundational libraries such as this to expose APIs that enable us
//! to distinguish between regional preferences values derived by the host from defaults of a locale, from cases
//! when the value is explicitly set by the user.
//! This distinction allows ICU to better serve in locale negotiations scenario where other-than-first locale may be used
//! and the deployment should respect whether the user set a given preference explicitly or left it to the per-locale default.
//!
//! [`icu`]: https://docs.rs/icu/latest/icu/
//! [`Unicode`]: https://docs.rs/icu_locale_core/latest/icu_locale_core/extensions/unicode/struct.Unicode.html
//! [`Preferences`]: https://docs.rs/icu_locale_core/latest/icu_locale_core/preferences/index.html
//! [`DateTimeFormatter`]: https://docs.rs/icu_datetime/latest/icu_datetime/struct.DateTimeFormatter.html
pub mod backends;
mod error;
mod host_info;
pub mod locale;

pub use host_info::*;

/// Enumeration of known hosts.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[non_exhaustive]
pub enum HostKind {
    Android,
    Ios,
    Linux,
    MacOS,
    Windows,
}
