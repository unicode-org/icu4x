// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This API provides necessary functionality for building user preferences structs.
//!
//! It includes the ability to merge information between the struct and a [`Locale`],
//! facilitating the resolution of attributes against default values.
//!
//! Preferences struct serve as a composable argument to `ICU4X` constructors, allowing
//! for ergonomic merging between information encoded in multiple sets of user inputs:
//! Locale, application preferences and operating system preferences.
//!
//! The crate is intended primarily to be used by components constructors to normalize the format
//! of ingesting preferences across all of `ICU4X`.
//!
//! # Preferences vs Options
//!
//! ICU4X introduces a separation between two classes of parameters that are used
//! to adjust the behavior of a component.
//!
//! `Preferences` represent the user-driven preferences on how the given user wants the internationalization
//! to behave. Those are items like language, script, calendar and numbering systems etc.
//!
//! `Options` represent the developer-driven adjustments that affect how given information is presented
//! based on the requirements of the application like available space or intended tone.
//!
//! # Options Division
//!
//! The `Options` themselves are also divided into options that are affecting data slicing, and ones that don't.
//! This is necessary to allow for DCE and FFI to produce minimal outputs avoiding loading unnecessary data that
//! is never to be used by a given component.
//! The result is that some option keys affect specialized constructors such as `try_new_short`, `try_new_long`, which
//! result in data provider loading only data necessary to format short or long values respectively.
//! For options that are not affecting data slicing, an `Options` struct is provided that the developer
//! can fill with selected key values, or use the defaults.
//!
//! # Preferences Merging
//!
//! In traditional internatonalization APIs, the argument passed to constructors is a locale.
//! ICU4X changes this paradigm by accepting a `Preferences`, which can be extracted from a [`Locale`] and combined with
//! other `Preferences`s provided by the environment.
//!
//! This approach makes it easy for developers to write code that takes just a locale, as in other systems,
//! as well as handle more sophisticated cases where the application may receive, for example, a locale,
//! a set of internationalization preferences specified within the application,
//! and a third set extracted from the operating system's preferences.
//!
//! # ECMA-402 vs ICU4X
//!
//! The result of the two paradigm shifts presented above is that the way constructors work is different.
//!
//! ## ECMA-402
//! ```ignore
//! let locale = new Locale("en-US-u-hc-h12");
//! let options = {
//!   hourCycle: "h24", // user preference
//!   timeStyle: "long", // developer option
//! };
//!
//! let dtf = new DateTimeFormat(locale, options);
//! ```
//!
//! ## ICU4X
//! ```ignore
//! let loc = locale!("en-US-u-hc-h12");
//! let prefs = DateTimeFormatterPreferences {
//!     hour_cycle: HourCycle::H24,
//! };
//! let options = DateTimeFormatterOptions {
//!     time_style: TimeStyle::Long,
//! };
//!
//! let mut combined_prefs = DateTimeFormatterPreferences::from(loc);
//! combined_prefs.extend(prefs);
//!
//! let dtf = DateTimeFormatter::try_new(combined_prefs, options);
//! ```
//!
//! This architecture allows for flexible composition of user and developer settings
//! sourced from different locations in custom ways based on the needs of each deployment.
//!
//! Below are some examples of how the `Preferences` model can be used in different setups.
//!
//! # Examples
//!
//! ```
//! use icu::locale::preferences::{
//!   define_preferences,
//!   extensions::unicode::keywords::HourCycle,
//! };
//! use icu::locale::locale;
//!
//! # fn get_data_locale_from_prefs(input: ExampleComponentPreferences) -> () { () }
//! # fn load_data(locale: ()) -> MyData { MyData {} }
//! # struct MyData {}
//! define_preferences!(
//!     // Name of the preferences struct
//!     ExampleComponentPreferences,
//!     {
//!         // A preference relevant to the component
//!         hour_cycle: HourCycle
//!     }
//! );
//!
//! pub struct ExampleComponent {
//!     data: MyData,
//! }
//!
//! impl ExampleComponent {
//!     pub fn new(prefs: ExampleComponentPreferences) -> Self {
//!         let locale = get_data_locale_from_prefs(prefs);
//!         let data = load_data(locale);
//!
//!         Self { data }
//!     }
//! }
//! ```
//!
//! Now we can use that component in multiple different ways,
//!
//! ## Scenario 1: Use Locale as the only input
//! ```
//! # use icu::locale::preferences::{
//! #   define_preferences,
//! #   extensions::unicode::keywords::HourCycle,
//! # };
//! # use icu::locale::locale;
//! # fn get_data_locale_from_prefs(input: ExampleComponentPreferences) -> () { () }
//! # fn load_data(locale: ()) -> MyData { MyData {} }
//! # struct MyData {}
//! # define_preferences!(
//! #     // Name of the preferences struct
//! #     ExampleComponentPreferences,
//! #     {
//! #         // A preference relevant to the component
//! #         hour_cycle: HourCycle
//! #     }
//! # );
//! #
//! # pub struct ExampleComponent {
//! #     data: MyData,
//! # }
//! # impl ExampleComponent {
//! #     pub fn new(prefs: ExampleComponentPreferences) -> Self {
//! #         let locale = get_data_locale_from_prefs(prefs);
//! #         let data = load_data(locale);
//! #
//! #         Self { data }
//! #     }
//! # }
//! let loc = locale!("en-US-u-hc-h23");
//! let tf = ExampleComponent::new(loc.into());
//! ```
//!
//! ## Scenario 2: Compose Preferences and Locale
//! ```
//! # use icu::locale::preferences::{
//! #   define_preferences,
//! #   extensions::unicode::keywords::HourCycle,
//! # };
//! # use icu::locale::locale;
//! # fn get_data_locale_from_prefs(input: ExampleComponentPreferences) -> () { () }
//! # fn load_data(locale: ()) -> MyData { MyData {} }
//! # struct MyData {}
//! # define_preferences!(
//! #     // Name of the preferences struct
//! #     ExampleComponentPreferences,
//! #     {
//! #         // A preference relevant to the component
//! #         hour_cycle: HourCycle
//! #     }
//! # );
//! #
//! # pub struct ExampleComponent {
//! #     data: MyData,
//! # }
//! # impl ExampleComponent {
//! #     pub fn new(prefs: ExampleComponentPreferences) -> Self {
//! #         let locale = get_data_locale_from_prefs(prefs);
//! #         let data = load_data(locale);
//! #
//! #         Self { data }
//! #     }
//! # }
//! let loc = locale!("en-US-u-hc-h23");
//! let app_prefs = ExampleComponentPreferences {
//!     hour_cycle: Some(HourCycle::H12),
//!     ..Default::default()
//! };
//!
//! let mut combined_prefs = ExampleComponentPreferences::from(loc);
//! combined_prefs.extend(app_prefs);
//!
//! // HourCycle is set from the prefs bag and override the value from the locale
//! assert_eq!(combined_prefs.hour_cycle, Some(HourCycle::H12));
//!
//! let tf = ExampleComponent::new(combined_prefs);
//! ```
//!
//! ## Scenario 3: Merge Preferences from Locale, OS, and Application
//! ```
//! # use icu::locale::preferences::{
//! #   define_preferences,
//! #   extensions::unicode::keywords::HourCycle,
//! # };
//! # use icu::locale::locale;
//! # fn get_data_locale_from_prefs(input: ExampleComponentPreferences) -> () { () }
//! # fn load_data(locale: ()) -> MyData { MyData {} }
//! # struct MyData {}
//! # define_preferences!(
//! #     // Name of the preferences struct
//! #     ExampleComponentPreferences,
//! #     {
//! #         // A preference relevant to the component
//! #         hour_cycle: HourCycle
//! #     }
//! # );
//! #
//! # pub struct ExampleComponent {
//! #     data: MyData,
//! # }
//! # impl ExampleComponent {
//! #     pub fn new(prefs: ExampleComponentPreferences) -> Self {
//! #         let locale = get_data_locale_from_prefs(prefs);
//! #         let data = load_data(locale);
//! #
//! #         Self { data }
//! #     }
//! # }
//! let loc = locale!("en-US-u-hc-h24");
//!
//! // Simulate OS preferences
//! let os_prefs = ExampleComponentPreferences {
//!     hour_cycle: Some(HourCycle::H23),
//!     ..Default::default()
//! };
//!
//! // Application does not specify hour_cycle
//! let app_prefs = ExampleComponentPreferences {
//!     hour_cycle: Some(HourCycle::H12),
//!     ..Default::default()
//! };
//!
//! let mut combined_prefs = ExampleComponentPreferences::from(loc);
//! combined_prefs.extend(os_prefs);
//! combined_prefs.extend(app_prefs);
//!
//! // HourCycle is set from the OS preferences since the application didn't specify it
//! assert_eq!(combined_prefs.hour_cycle, Some(HourCycle::H12));
//!
//! let tf = ExampleComponent::new(combined_prefs);
//! ```
//!
//! ## Scenario 4: Neither Application nor OS specify the preference
//! ```
//! # use icu::locale::preferences::{
//! #   define_preferences,
//! #   extensions::unicode::keywords::HourCycle,
//! # };
//! # use icu::locale::locale;
//! # fn get_data_locale_from_prefs(input: ExampleComponentPreferences) -> () { () }
//! # fn load_data(locale: ()) -> MyData { MyData {} }
//! # struct MyData {}
//! # define_preferences!(
//! #     // Name of the preferences struct
//! #     ExampleComponentPreferences,
//! #     {
//! #         // A preference relevant to the component
//! #         hour_cycle: HourCycle
//! #     }
//! # );
//! #
//! # pub struct ExampleComponent {
//! #     data: MyData,
//! # }
//! # impl ExampleComponent {
//! #     pub fn new(prefs: ExampleComponentPreferences) -> Self {
//! #         let locale = get_data_locale_from_prefs(prefs);
//! #         let data = load_data(locale);
//! #
//! #         Self { data }
//! #     }
//! # }
//! let loc = locale!("en-US-u-hc-h24");
//!
//! // Simulate OS preferences
//! let os_prefs = ExampleComponentPreferences::default(); // OS does not specify hour_cycle
//! let app_prefs = ExampleComponentPreferences::default(); // Application does not specify hour_cycle
//!
//! let mut combined_prefs = ExampleComponentPreferences::from(loc);
//! combined_prefs.extend(os_prefs);
//! combined_prefs.extend(app_prefs);
//!
//! // HourCycle is taken from the locale
//! assert_eq!(combined_prefs.hour_cycle, Some(HourCycle::H24));
//!
//! let tf = ExampleComponent::new(combined_prefs);
//! ```
//!
//! [`ICU4X`]: ../icu/index.html
//! [`Locale`]: crate::Locale

pub mod extensions;

/// A low-level trait implemented on each preference exposed in component preferences.
///
/// [`PreferenceKey`] has to be implemented on
/// preferences that are to be included in Formatter preferences.
/// The trait may be implemented to indicate that the given preference has
/// a unicode key corresponding to it or be a custom one.
///
/// `ICU4X` provides an implementation of [`PreferenceKey`] for all
/// Unicode Extension Keys. The only external use of this trait is to implement
/// it on custom preferences that are to be included in a component preferences bag.
///
/// The below example show cases a manual generation of an `em` (emoji) unicode extension key
/// and a custom struct to showcase the difference in their behavior. For all use purposes,
/// the [`EmojiPresentationStyle`](crate::preferences::extensions::unicode::keywords::EmojiPresentationStyle) preference exposed by this crate should be used.
///
/// # Examples
/// ```
/// use icu::locale::{
///   extensions::unicode::{key, Key, value, Value},
///   preferences::{
///     define_preferences, PreferenceKey,
///     extensions::unicode::errors::PreferencesParseError,
///   },
/// };
///
/// #[non_exhaustive]
/// #[derive(Debug, Clone, Eq, PartialEq, Copy)]
/// pub enum EmojiPresentationStyle {
///     Emoji,
///     Text,
///     Default,
/// }
///
/// impl PreferenceKey for EmojiPresentationStyle {
///     fn unicode_extension_key() -> Option<Key> {
///         Some(key!("em"))
///     }
///
///     fn try_from_key_value(
///         key: &Key,
///         value: &Value,
///     ) -> Result<Option<Self>, PreferencesParseError> {
///         if Self::unicode_extension_key() == Some(*key) {
///             let subtag = value.as_single_subtag()
///                               .ok_or(PreferencesParseError::InvalidKeywordValue)?;
///             match subtag.as_str() {
///                 "emoji" => Ok(Some(Self::Emoji)),
///                 "text" => Ok(Some(Self::Text)),
///                 "default" => Ok(Some(Self::Default)),
///                 _ => Err(PreferencesParseError::InvalidKeywordValue)
///             }
///         } else {
///             Ok(None)
///         }
///     }
///
///     fn unicode_extension_value(&self) -> Option<Value> {
///         Some(match self {
///             EmojiPresentationStyle::Emoji => value!("emoji"),
///             EmojiPresentationStyle::Text => value!("text"),
///             EmojiPresentationStyle::Default => value!("default"),
///         })
///     }
/// }
///
/// #[non_exhaustive]
/// #[derive(Debug, Clone, Eq, PartialEq)]
/// pub struct CustomFormat {
///     value: String
/// }
///
/// impl PreferenceKey for CustomFormat {}
///
/// define_preferences!(
///     MyFormatterPreferences,
///     {
///         emoji: EmojiPresentationStyle,
///         custom: CustomFormat
///     }
/// );
/// ```
/// [`ICU4X`]: ../icu/index.html
pub trait PreferenceKey: Sized {
    /// Optional constructor of the given preference. It takes the
    /// unicode extension key and if the key matches it attemptes to construct
    /// the preference based on the given value.
    /// If the value is not a valid value for the given key, the constructor throws.
    fn try_from_key_value(
        _key: &crate::extensions::unicode::Key,
        _value: &crate::extensions::unicode::Value,
    ) -> Result<Option<Self>, crate::preferences::extensions::unicode::errors::PreferencesParseError>
    {
        Ok(None)
    }

    /// Retrieve unicode extension key corresponding to a given preference.
    fn unicode_extension_key() -> Option<crate::extensions::unicode::Key> {
        None
    }

    /// Retrieve unicode extension value corresponding to the given instance of the preference.
    fn unicode_extension_value(&self) -> Option<crate::extensions::unicode::Value> {
        None
    }
}

/// A macro to facilitate generation of preferences struct.
///
///
/// The generated preferences struct provides methods for merging and converting between [`Locale`] and
/// the preference bag. See [`preferences`](crate::preferences) for use cases.
///
/// In the example below, the input argument is the generated preferences struct which
/// can be auto-converted from a Locale, or combined from a Locale and Preferences Bag.
///
/// # Examples
/// ```
/// use icu::locale::{
///     preferences::{
///         define_preferences,
///         extensions::unicode::keywords::HourCycle
///     },
///     locale,
/// };
///
/// define_preferences!(
///     TimeFormatterPreferences,
///     {
///         hour_cycle: HourCycle
///     }
/// );
///
/// struct TimeFormatter {}
///
/// impl TimeFormatter {
///     pub fn try_new(prefs: TimeFormatterPreferences) -> Result<Self, ()> {
///         // load data and set struct fields based on the prefs input
///         Ok(Self {})
///     }
/// }
///
/// let loc = locale!("en-US");
///
/// let tf = TimeFormatter::try_new(loc.into());
/// ```
///
/// [`Locale`]: crate::Locale
#[macro_export]
#[doc(hidden)]
macro_rules! __define_preferences {
    (
        $(#[$doc:meta])*
        $name:ident,
        {
            $(
                $(#[$key_doc:meta])*
                $key:ident: $pref:ty
            ),*
        }
     ) => (
        $(#[$doc])*
        #[derive(Default, Debug, Clone)]
        #[non_exhaustive]
        pub struct $name {
            pub(crate) language: $crate::subtags::Language,
            pub(crate) script: Option<$crate::subtags::Script>,
            pub(crate) region: Option<$crate::subtags::Region>,
            pub(crate) variant: Option<$crate::subtags::Variant>,
            pub(crate) subdivision: Option<$crate::subtags::Subtag>,
            pub(crate) ue_region: Option<$crate::subtags::Region>,

            $(
                $(#[$key_doc])*
                pub $key: Option<$pref>,
            )*
        }

        impl From<$crate::Locale> for $name {
            fn from(loc: $crate::Locale) -> Self {
                $name::from(&loc)
            }
        }

        impl From<&$crate::Locale> for $name {
            fn from(loc: &$crate::Locale) -> Self {
                use $crate::preferences::PreferenceKey;

                $(
                    let mut $key = None;
                )*

                for (k, v) in loc.extensions.unicode.keywords.iter() {
                    $(
                        if let Ok(Some(r)) = <$pref>::try_from_key_value(k, v) {
                            $key = Some(r);
                            continue;
                        }
                    )*
                }

                let sd = loc
                    .extensions
                    .unicode
                    .keywords
                    .get(&$crate::extensions::unicode::key!("sd"))
                    .and_then(|v| v.as_single_subtag().copied());
                let ue_region = loc
                        .extensions
                        .unicode
                        .keywords
                        .get(&$crate::extensions::unicode::key!("rg"))
                        .and_then(|v| v.as_single_subtag()
                            .and_then(|s| $crate::subtags::Region::try_from_str(s.as_str()).ok()));
                Self {
                    language: loc.id.language,
                    script: loc.id.script,
                    region: loc.id.region,
                    variant: loc.id.variants.iter().copied().next(),
                    subdivision: sd,
                    ue_region,

                    $(
                        $key,
                    )*
                }
            }
        }

        impl From<$crate::LanguageIdentifier> for $name {
            fn from(lid: $crate::LanguageIdentifier) -> Self {
                $name::from(&lid)
            }
        }

        impl From<&$crate::LanguageIdentifier> for $name {
            fn from(lid: &$crate::LanguageIdentifier) -> Self {
                Self {
                    language: lid.language,
                    script: lid.script,
                    region: lid.region,
                    variant: lid.variants.iter().copied().next(),
                    subdivision: None,
                    ue_region: None,

                    $(
                        $key: None,
                    )*
                }
            }
        }

        impl From<&icu_provider::DataLocale> for $name {
            fn from(loc: &icu_provider::DataLocale) -> Self {
                Self {
                    language: loc.language,
                    script: loc.script,
                    region: loc.region,
                    variant: loc.variant,
                    subdivision: loc.subdivision,
                    ue_region: None,

                    $(
                        $key: None,
                    )*
                }
            }
        }

        impl $name {
            /// Constructs a `Locale` corresponding to these preferences.
            pub fn into_locale(self) -> $crate::Locale {
                use $crate::preferences::PreferenceKey;
                $crate::Locale {
                    id: $crate::LanguageIdentifier {
                        language: self.language,
                        script: self.script,
                        region: self.region,
                        variants: self
                            .variant
                            .map($crate::subtags::Variants::from_variant)
                            .unwrap_or_default(),
                    },
                    extensions: {
                        let mut extensions = $crate::extensions::Extensions::default();
                        if self.subdivision.is_some() || self.ue_region.is_some() {
                            if let Some(sd) = self.subdivision {
                                extensions.unicode.keywords.set(
                                    $crate::extensions::unicode::key!("sd"),
                                    $crate::extensions::unicode::Value::from_subtag(Some(sd))
                                );
                            }
                            if let Some(rg) = self.ue_region {
                                extensions.unicode.keywords.set(
                                    $crate::extensions::unicode::key!("rg"),
                                    $crate::extensions::unicode::Value::try_from_str(rg.as_str()).unwrap()
                                );
                            }
                        }
                        $(
                            if let Some(value) = &self.$key {
                                if let Some(ue) = <$pref>::unicode_extension_key() {
                                    let val = value.unicode_extension_value().unwrap();
                                    extensions.unicode.keywords.set(ue, val);
                                }
                            }
                        )*
                        extensions
                    },
                }
            }

            /// Extends the preferences with the values from another set of preferences.
            pub fn extend(&mut self, other: $name) {
                $(
                    if let Some(value) = other.$key {
                        self.$key = Some(value);
                    }
                )*
            }
        }
    )
}
#[doc(inline)]
pub use __define_preferences as define_preferences;
