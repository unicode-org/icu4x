// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module defines all available properties.
//!
//! Properties may be empty marker types and implement [`BinaryProperty`], or enumerations[^1]
//! and implement [`EnumeratedProperty`].
//!
//! [`BinaryProperty`]s are queried through a [`CodePointSetData`](crate::CodePointSetData),
//! while [`EnumeratedProperty`]s are queried through [`CodePointMapData`](crate::CodePointMapData).
//!
//! In addition, some [`EnumeratedProperty`]s also implement [`ParseableEnumeratedProperty`] or
//! [`NamedEnumeratedProperty`]. For these properties, [`PropertyParser`](crate::PropertyParser),
//! [`PropertyNamesLong`](crate::PropertyNamesLong), and [`PropertyNamesShort`](crate::PropertyNamesShort)
//! can be constructed.
//!
//! [^1]: either Rust `enum`s, or Rust `struct`s with associated constants (open enums)

pub use crate::names::{NamedEnumeratedProperty, ParseableEnumeratedProperty};

pub use crate::bidi::{BidiMirroringGlyph, BidiPairedBracketType};

/// See [`test_enumerated_property_completeness`] for usage.
/// Example input:
/// ```ignore
/// impl EastAsianWidth {
///     pub const Neutral: EastAsianWidth = EastAsianWidth(0);
///     pub const Ambiguous: EastAsianWidth = EastAsianWidth(1);
///     ...
/// }
/// ```
/// Produces `const ALL_VALUES = &[("Neutral", 0u16), ...];` by
/// explicitly casting first field of the struct to u16.
macro_rules! create_const_array {
    (
        $ ( #[$meta:meta] )*
        impl $enum_ty:ident {
            #[default]
            $(#[$default_meta:meta])* $dv:vis const $di:ident: $dt:ty = $de:expr; / $default_short_name:literal
            $( $(#[$const_meta:meta])* $v:vis const $i:ident: $t:ty = $e:expr; / $short_name:literal )*
            $(
                $additional_name:literal => $additonal_discriminant:expr;
            )*
        }
        #[test]
        fn $consts_test:ident();
    ) => {
        $( #[$meta] )*
        impl $enum_ty {
            $(#[$default_meta])*
            $dv const $di: $dt = $de;
            $(
                $(#[$const_meta])*
                $v const $i: $t = $e;
            )*

            /// All possible values of this enum in the Unicode version
            /// from this ICU4X release.
            pub const ALL_VALUES: &'static [$enum_ty] = &[
                $enum_ty::$di,
                $($enum_ty::$i),*
            ];
        }

        impl Default for $enum_ty {
            fn default() -> Self {
                Self::$di
            }
        }

        #[cfg(feature = "datagen")]
        impl databake::Bake for $enum_ty {
            fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                env.insert("icu_properties");
                match *self {
                    $enum_ty::$di => databake::quote!(icu_properties::props::$enum_ty::$di),
                    $(
                        Self::$i => databake::quote!(icu_properties::props::$enum_ty::$i),
                    )*
                    Self(v) => databake::quote!(icu_properties::props::$enum_ty(#v)),
                }
            }
        }


        impl From<$enum_ty> for u16  {
            #[allow(trivial_numeric_casts)]
            fn from(other: $enum_ty) -> Self {
                other.0 as u16
            }
        }

        impl $enum_ty {
            #[cfg(feature = "datagen")]
            #[doc(hidden)]
            pub fn names() -> impl Iterator<Item = (&'static str, Self)> {
                [
                    ($default_short_name, Self::$di),
                    $(
                        ($short_name, Self::$i),
                    )*
                    $(
                        ($additional_name, Self($additonal_discriminant)),
                    )*
                ].into_iter()
            }
        }

        #[test]
        fn $consts_test() {
            assert_eq!(
                crate::names::PropertyNamesLong::<$enum_ty>::new().get($enum_ty::$di).unwrap().replace('_', ""),
                stringify!($di)
            );
            $(
                assert_eq!(
                    crate::names::PropertyNamesLong::<$enum_ty>::new().get($enum_ty::$i).unwrap()
                        // Rust identifiers use camel case
                        .replace('_', "")
                        // We use Ethiopian
                        .replace("Ethiopic", "Ethiopian")
                        // Nastaliq is missing a long name?
                        .replace("Aran", "Nastaliq")
                        // We spell these out
                        .replace("LVSyllable", "LeadingVowelSyllable")
                        .replace("LVTSyllable", "LeadingVowelTrailingSyllable"),
                    stringify!($i)
                );
            )*
        }
    }
}

pub use crate::code_point_map::EnumeratedProperty;

macro_rules! make_enumerated_property {
    (
        name: $name:literal;
        short_name: $short_name:literal;
        ident: $value_ty:path;
        data_marker: $data_marker:ty;
        singleton: $singleton:ident;
        $(ule_ty: $ule_ty:ty;)?
    ) => {
        impl crate::private::Sealed for $value_ty {}

        impl EnumeratedProperty for $value_ty {
            type DataMarker = $data_marker;
            #[cfg(feature = "compiled_data")]
            const SINGLETON: &'static crate::provider::PropertyCodePointMap<'static, Self> =
                crate::provider::Baked::$singleton;
            const NAME: &'static [u8] = $name.as_bytes();
            const SHORT_NAME: &'static [u8] = $short_name.as_bytes();
        }

        $(
            impl zerovec::ule::AsULE for $value_ty {
                type ULE = $ule_ty;

                fn to_unaligned(self) -> Self::ULE {
                    self.0.to_unaligned()
                }
                fn from_unaligned(unaligned: Self::ULE) -> Self {
                    Self(zerovec::ule::AsULE::from_unaligned(unaligned))
                }
            }
        )?
    };
}

/// Enumerated property `Bidi_Class`
///
/// These are the categories required by the Unicode Bidirectional Algorithm.
/// For the property values, see [Bidirectional Class Values](https://unicode.org/reports/tr44/#Bidi_Class_Values).
/// For more information, see [Unicode Standard Annex #9](https://unicode.org/reports/tr41/tr41-28.html#UAX9).
///
/// # Example
///
/// ```
/// use icu::properties::{props::BidiClass, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<BidiClass>::new().get('y'),
///     BidiClass::LeftToRight
/// ); // U+0079
/// assert_eq!(
///     CodePointMapData::<BidiClass>::new().get('ع'),
///     BidiClass::ArabicLetter
/// ); // U+0639
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct BidiClass(pub(crate) u8);

impl BidiClass {
    /// Returns an ICU4C `UBidiClass` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UBidiClass` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(non_upper_case_globals)]
impl BidiClass {
    #[default]
    /// (`L`) any strong left-to-right character
    pub const LeftToRight: BidiClass = BidiClass(0); / "L"
    /// (`R`) any strong right-to-left (non-Arabic-type) character
    pub const RightToLeft: BidiClass = BidiClass(1); / "R"
    /// (`EN`) any ASCII digit or Eastern Arabic-Indic digit
    pub const EuropeanNumber: BidiClass = BidiClass(2); / "EN"
    /// (`ES`) plus and minus signs
    pub const EuropeanSeparator: BidiClass = BidiClass(3); / "ES"
    /// (`ET`) a terminator in a numeric format context, includes currency signs
    pub const EuropeanTerminator: BidiClass = BidiClass(4); / "ET"
    /// (`AN`) any Arabic-Indic digit
    pub const ArabicNumber: BidiClass = BidiClass(5); / "AN"
    /// (`CS`) commas, colons, and slashes
    pub const CommonSeparator: BidiClass = BidiClass(6); / "CS"
    /// (`B`) various newline characters
    pub const ParagraphSeparator: BidiClass = BidiClass(7); / "B"
    /// (`S`) various segment-related control codes
    pub const SegmentSeparator: BidiClass = BidiClass(8); / "S"
    /// (`WS`) spaces
    pub const WhiteSpace: BidiClass = BidiClass(9); / "WS"
    /// (`ON`) most other symbols and punctuation marks
    pub const OtherNeutral: BidiClass = BidiClass(10); / "ON"
    /// (`LRE`) U+202A: the LR embedding control
    pub const LeftToRightEmbedding: BidiClass = BidiClass(11); / "LRE"
    /// (`LRO`) U+202D: the LR override control
    pub const LeftToRightOverride: BidiClass = BidiClass(12); / "LRO"
    /// (`AL`) any strong right-to-left (Arabic-type) character
    pub const ArabicLetter: BidiClass = BidiClass(13); / "AL"
    /// (`RLE`) U+202B: the RL embedding control
    pub const RightToLeftEmbedding: BidiClass = BidiClass(14); / "RLE"
    /// (`RLO`) U+202E: the RL override control
    pub const RightToLeftOverride: BidiClass = BidiClass(15); / "RLO"
    /// (`PDF`) U+202C: terminates an embedding or override control
    pub const PopDirectionalFormat: BidiClass = BidiClass(16); / "PDF"
    /// (`NSM`) any nonspacing mark
    pub const NonspacingMark: BidiClass = BidiClass(17); / "NSM"
    /// (`BN`) most format characters, control codes, or noncharacters
    pub const BoundaryNeutral: BidiClass = BidiClass(18); / "BN"
    /// (`FSI`) U+2068: the first strong isolate control
    pub const FirstStrongIsolate: BidiClass = BidiClass(19); / "FSI"
    /// (`LRI`) U+2066: the LR isolate control
    pub const LeftToRightIsolate: BidiClass = BidiClass(20); / "LRI"
    /// (`RLI`) U+2067: the RL isolate control
    pub const RightToLeftIsolate: BidiClass = BidiClass(21); / "RLI"
    /// (`PDI`) U+2069: terminates an isolate control
    pub const PopDirectionalIsolate: BidiClass = BidiClass(22); / "PDI"
}
#[test]
fn bidi_props_consts();
}

make_enumerated_property! {
    name: "Bidi_Class";
    short_name: "bc";
    ident: BidiClass;
    data_marker: crate::provider::PropertyEnumBidiClassV1;
    singleton: SINGLETON_PROPERTY_ENUM_BIDI_CLASS_V1;
    ule_ty: u8;
}

/// Enumerated property `Numeric_Type`.
///
/// See Section 4.6, Numeric Value in The Unicode Standard for the summary of
/// each property value.
///
/// # Example
///
/// ```
/// use icu::properties::{props::NumericType, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<NumericType>::new().get('0'),
///     NumericType::Decimal,
/// ); // U+0030
/// assert_eq!(
///     CodePointMapData::<NumericType>::new().get('½'),
///     NumericType::Numeric,
/// ); // U+00BD
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct NumericType(pub(crate) u8);

impl NumericType {
    /// Returns an ICU4C `UNumericType` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UNumericType` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(non_upper_case_globals)]
impl NumericType {
    #[default]
    /// Characters without numeric value
    pub const None: NumericType = NumericType(0); / "None"
    /// (`De`) Characters of positional decimal systems
    ///
    /// These are coextensive with [`GeneralCategory::DecimalNumber`].
    pub const Decimal: NumericType = NumericType(1); / "De"
    /// (`Di`) Variants of positional or sequences thereof.
    ///
    /// The distinction between [`NumericType::Digit`] and [`NumericType::Numeric`]
    /// has not proven to be useful, so no further characters will be added to
    /// this type.
    pub const Digit: NumericType = NumericType(2); / "Di"
    /// (`Nu`) Other characters with numeric value
    pub const Numeric: NumericType = NumericType(3); / "Nu"
}
#[test]
fn numeric_type_consts();
}

make_enumerated_property! {
    name: "Numeric_Type";
    short_name: "nt";
    ident: NumericType;
    data_marker: crate::provider::PropertyEnumNumericTypeV1;
    singleton: SINGLETON_PROPERTY_ENUM_NUMERIC_TYPE_V1;
    ule_ty: u8;
}

// This exists to encapsulate GeneralCategoryULE so that it can exist in the provider module rather than props
pub(crate) mod gc {
    /// Enumerated property `General_Category`.
    ///
    /// `General_Category` specifies the most general classification of a code point, usually
    /// determined based on the primary characteristic of the assigned character. For example, is the
    /// character a letter, a mark, a number, punctuation, or a symbol, and if so, of what type?
    ///
    /// `GeneralCategory` only supports specific subcategories (eg `UppercaseLetter`).
    /// It does not support grouped categories (eg `Letter`). For grouped categories, use [`GeneralCategoryGroup`](
    /// crate::props::GeneralCategoryGroup).
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{props::GeneralCategory, CodePointMapData};
    ///
    /// assert_eq!(
    ///     CodePointMapData::<GeneralCategory>::new().get('木'),
    ///     GeneralCategory::OtherLetter
    /// ); // U+6728
    /// assert_eq!(
    ///     CodePointMapData::<GeneralCategory>::new().get('🎃'),
    ///     GeneralCategory::OtherSymbol
    /// ); // U+1F383 JACK-O-LANTERN
    /// ```
    #[derive(Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash, Default)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "datagen", derive(databake::Bake))]
    #[cfg_attr(feature = "datagen", databake(path = icu_properties::props))]
    #[allow(clippy::exhaustive_enums)] // this type is stable
    #[zerovec::make_ule(GeneralCategoryULE)]
    #[cfg_attr(not(feature = "alloc"), zerovec::skip_derive(ZeroMapKV))]
    #[repr(u8)]
    pub enum GeneralCategory {
        #[default]
        /// (`Cn`) A reserved unassigned code point or a noncharacter
        Unassigned = 0,

        /// (`Lu`) An uppercase letter
        UppercaseLetter = 1,
        /// (`Ll`) A lowercase letter
        LowercaseLetter = 2,
        /// (`Lt`) A digraphic letter, with first part uppercase
        TitlecaseLetter = 3,
        /// (`Lm`) A modifier letter
        ModifierLetter = 4,
        /// (`Lo`) Other letters, including syllables and ideographs
        OtherLetter = 5,

        /// (`Mn`) A nonspacing combining mark (zero advance width)
        NonspacingMark = 6,
        /// (`Mc`) A spacing combining mark (positive advance width)
        SpacingMark = 8,
        /// (`Me`) An enclosing combining mark
        EnclosingMark = 7,

        /// (`Nd`) A decimal digit
        DecimalNumber = 9,
        /// (`Nl`) A letterlike numeric character
        LetterNumber = 10,
        /// (`No`) A numeric character of other type
        OtherNumber = 11,

        /// (`Zs`) A space character (of various non-zero widths)
        SpaceSeparator = 12,
        /// (`Zl`) U+2028 LINE SEPARATOR only
        LineSeparator = 13,
        /// (`Zp`) U+2029 PARAGRAPH SEPARATOR only
        ParagraphSeparator = 14,

        /// (`Cc`) A C0 or C1 control code
        Control = 15,
        /// (`Cf`) A format control character
        Format = 16,
        /// (`Co`) A private-use character
        PrivateUse = 17,
        /// (`Cs`) A surrogate code point
        Surrogate = 18,

        /// (`Pd`) A dash or hyphen punctuation mark
        DashPunctuation = 19,
        /// (`Ps`) An opening punctuation mark (of a pair)
        OpenPunctuation = 20,
        /// (`Pe`) A closing punctuation mark (of a pair)
        ClosePunctuation = 21,
        /// (`Pc`) A connecting punctuation mark, like a tie
        ConnectorPunctuation = 22,
        /// (`Pi`) An initial quotation mark
        InitialPunctuation = 28,
        /// (`Pf`) A final quotation mark
        FinalPunctuation = 29,
        /// (`Po`) A punctuation mark of other type
        OtherPunctuation = 23,

        /// (`Sm`) A symbol of mathematical use
        MathSymbol = 24,
        /// (`Sc`) A currency sign
        CurrencySymbol = 25,
        /// (`Sk`) A non-letterlike modifier symbol
        ModifierSymbol = 26,
        /// (`So`) A symbol of other type
        OtherSymbol = 27,
    }
}

pub use gc::GeneralCategory;

impl GeneralCategory {
    /// All possible values of this enum
    pub const ALL_VALUES: &'static [GeneralCategory] = &[
        GeneralCategory::Unassigned,
        GeneralCategory::UppercaseLetter,
        GeneralCategory::LowercaseLetter,
        GeneralCategory::TitlecaseLetter,
        GeneralCategory::ModifierLetter,
        GeneralCategory::OtherLetter,
        GeneralCategory::NonspacingMark,
        GeneralCategory::SpacingMark,
        GeneralCategory::EnclosingMark,
        GeneralCategory::DecimalNumber,
        GeneralCategory::LetterNumber,
        GeneralCategory::OtherNumber,
        GeneralCategory::SpaceSeparator,
        GeneralCategory::LineSeparator,
        GeneralCategory::ParagraphSeparator,
        GeneralCategory::Control,
        GeneralCategory::Format,
        GeneralCategory::PrivateUse,
        GeneralCategory::Surrogate,
        GeneralCategory::DashPunctuation,
        GeneralCategory::OpenPunctuation,
        GeneralCategory::ClosePunctuation,
        GeneralCategory::ConnectorPunctuation,
        GeneralCategory::InitialPunctuation,
        GeneralCategory::FinalPunctuation,
        GeneralCategory::OtherPunctuation,
        GeneralCategory::MathSymbol,
        GeneralCategory::CurrencySymbol,
        GeneralCategory::ModifierSymbol,
        GeneralCategory::OtherSymbol,
    ];

    #[cfg(feature = "datagen")]
    #[doc(hidden)]
    pub fn names() -> impl Iterator<Item = (&'static str, Self)> {
        [
            ("Cn", Self::Unassigned),
            ("Lu", Self::UppercaseLetter),
            ("Ll", Self::LowercaseLetter),
            ("Lt", Self::TitlecaseLetter),
            ("Lm", Self::ModifierLetter),
            ("Lo", Self::OtherLetter),
            ("Mn", Self::NonspacingMark),
            ("Mc", Self::SpacingMark),
            ("Me", Self::EnclosingMark),
            ("Nd", Self::DecimalNumber),
            ("Nl", Self::LetterNumber),
            ("No", Self::OtherNumber),
            ("Zs", Self::SpaceSeparator),
            ("Zl", Self::LineSeparator),
            ("Zp", Self::ParagraphSeparator),
            ("Cc", Self::Control),
            ("Cf", Self::Format),
            ("Co", Self::PrivateUse),
            ("Cs", Self::Surrogate),
            ("Pd", Self::DashPunctuation),
            ("Ps", Self::OpenPunctuation),
            ("Pe", Self::ClosePunctuation),
            ("Pc", Self::ConnectorPunctuation),
            ("Pi", Self::InitialPunctuation),
            ("Pf", Self::FinalPunctuation),
            ("Po", Self::OtherPunctuation),
            ("Sm", Self::MathSymbol),
            ("Sc", Self::CurrencySymbol),
            ("Sk", Self::ModifierSymbol),
            ("So", Self::OtherSymbol),
        ]
        .into_iter()
    }
}

#[test]
fn gc_variants() {
    for &variant in GeneralCategory::ALL_VALUES {
        assert_eq!(
            crate::names::PropertyNamesLong::<GeneralCategory>::new()
                .get(variant)
                .unwrap()
                // Rust identifiers use camel case
                .replace('_', ""),
            format!("{variant:?}")
        );
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
/// Error value for `impl TryFrom<u8> for GeneralCategory`.
#[non_exhaustive]
pub struct GeneralCategoryOutOfBoundsError;

impl TryFrom<u8> for GeneralCategory {
    type Error = GeneralCategoryOutOfBoundsError;
    /// Construct this [`GeneralCategory`] from an integer, returning
    /// an error if it is out of bounds
    fn try_from(val: u8) -> Result<Self, GeneralCategoryOutOfBoundsError> {
        GeneralCategory::new_from_u8(val).ok_or(GeneralCategoryOutOfBoundsError)
    }
}

make_enumerated_property! {
    name: "General_Category";
    short_name: "gc";
    ident: GeneralCategory;
    data_marker: crate::provider::PropertyEnumGeneralCategoryV1;
    singleton: SINGLETON_PROPERTY_ENUM_GENERAL_CATEGORY_V1;
}

/// Groupings of multiple `General_Category` property values.
///
/// Instances of `GeneralCategoryGroup` represent the defined multi-category
/// values that are useful for users in certain contexts, such as regex. In
/// other words, unlike [`GeneralCategory`], this supports groups of general
/// categories: for example, `Letter` /// is the union of `UppercaseLetter`,
/// `LowercaseLetter`, etc.
///
/// See <https://www.unicode.org/reports/tr44/> .
///
/// The discriminants correspond to the `U_GC_XX_MASK` constants in ICU4C.
/// Unlike [`GeneralCategory`], this supports groups of general categories: for example, `Letter`
/// is the union of `UppercaseLetter`, `LowercaseLetter`, etc.
///
/// See `UCharCategory` and `U_GET_GC_MASK` in ICU4C.
#[derive(Copy, Clone, PartialEq, Debug, Eq)]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct GeneralCategoryGroup(pub(crate) u32);

impl crate::private::Sealed for GeneralCategoryGroup {}

use GeneralCategory as GC;
use GeneralCategoryGroup as GCG;

#[allow(non_upper_case_globals)]
impl GeneralCategoryGroup {
    /// (`Lu`) An uppercase letter
    pub const UppercaseLetter: GeneralCategoryGroup = GCG(1 << (GC::UppercaseLetter as u32));
    /// (`Ll`) A lowercase letter
    pub const LowercaseLetter: GeneralCategoryGroup = GCG(1 << (GC::LowercaseLetter as u32));
    /// (`Lt`) A digraphic letter, with first part uppercase
    pub const TitlecaseLetter: GeneralCategoryGroup = GCG(1 << (GC::TitlecaseLetter as u32));
    /// (`Lm`) A modifier letter
    pub const ModifierLetter: GeneralCategoryGroup = GCG(1 << (GC::ModifierLetter as u32));
    /// (`Lo`) Other letters, including syllables and ideographs
    pub const OtherLetter: GeneralCategoryGroup = GCG(1 << (GC::OtherLetter as u32));
    /// (`LC`) The union of `UppercaseLetter`, `LowercaseLetter`, and `TitlecaseLetter`
    pub const CasedLetter: GeneralCategoryGroup = GCG((1 << (GC::UppercaseLetter as u32))
        | (1 << (GC::LowercaseLetter as u32))
        | (1 << (GC::TitlecaseLetter as u32)));
    /// (`L`) The union of all letter categories
    pub const Letter: GeneralCategoryGroup = GCG((1 << (GC::UppercaseLetter as u32))
        | (1 << (GC::LowercaseLetter as u32))
        | (1 << (GC::TitlecaseLetter as u32))
        | (1 << (GC::ModifierLetter as u32))
        | (1 << (GC::OtherLetter as u32)));

    /// (`Mn`) A nonspacing combining mark (zero advance width)
    pub const NonspacingMark: GeneralCategoryGroup = GCG(1 << (GC::NonspacingMark as u32));
    /// (`Me`) An enclosing combining mark
    pub const EnclosingMark: GeneralCategoryGroup = GCG(1 << (GC::EnclosingMark as u32));
    /// (`Mc`) A spacing combining mark (positive advance width)
    pub const SpacingMark: GeneralCategoryGroup = GCG(1 << (GC::SpacingMark as u32));
    /// (`M`) The union of all mark categories
    pub const Mark: GeneralCategoryGroup = GCG((1 << (GC::NonspacingMark as u32))
        | (1 << (GC::EnclosingMark as u32))
        | (1 << (GC::SpacingMark as u32)));

    /// (`Nd`) A decimal digit
    pub const DecimalNumber: GeneralCategoryGroup = GCG(1 << (GC::DecimalNumber as u32));
    /// (`Nl`) A letterlike numeric character
    pub const LetterNumber: GeneralCategoryGroup = GCG(1 << (GC::LetterNumber as u32));
    /// (`No`) A numeric character of other type
    pub const OtherNumber: GeneralCategoryGroup = GCG(1 << (GC::OtherNumber as u32));
    /// (`N`) The union of all number categories
    pub const Number: GeneralCategoryGroup = GCG((1 << (GC::DecimalNumber as u32))
        | (1 << (GC::LetterNumber as u32))
        | (1 << (GC::OtherNumber as u32)));

    /// (`Zs`) A space character (of various non-zero widths)
    pub const SpaceSeparator: GeneralCategoryGroup = GCG(1 << (GC::SpaceSeparator as u32));
    /// (`Zl`) U+2028 LINE SEPARATOR only
    pub const LineSeparator: GeneralCategoryGroup = GCG(1 << (GC::LineSeparator as u32));
    /// (`Zp`) U+2029 PARAGRAPH SEPARATOR only
    pub const ParagraphSeparator: GeneralCategoryGroup = GCG(1 << (GC::ParagraphSeparator as u32));
    /// (`Z`) The union of all separator categories
    pub const Separator: GeneralCategoryGroup = GCG((1 << (GC::SpaceSeparator as u32))
        | (1 << (GC::LineSeparator as u32))
        | (1 << (GC::ParagraphSeparator as u32)));

    /// (`Cc`) A C0 or C1 control code
    pub const Control: GeneralCategoryGroup = GCG(1 << (GC::Control as u32));
    /// (`Cf`) A format control character
    pub const Format: GeneralCategoryGroup = GCG(1 << (GC::Format as u32));
    /// (`Co`) A private-use character
    pub const PrivateUse: GeneralCategoryGroup = GCG(1 << (GC::PrivateUse as u32));
    /// (`Cs`) A surrogate code point
    pub const Surrogate: GeneralCategoryGroup = GCG(1 << (GC::Surrogate as u32));
    /// (`Cn`) A reserved unassigned code point or a noncharacter
    pub const Unassigned: GeneralCategoryGroup = GCG(1 << (GC::Unassigned as u32));
    /// (`C`) The union of all control code, reserved, and unassigned categories
    pub const Other: GeneralCategoryGroup = GCG((1 << (GC::Control as u32))
        | (1 << (GC::Format as u32))
        | (1 << (GC::PrivateUse as u32))
        | (1 << (GC::Surrogate as u32))
        | (1 << (GC::Unassigned as u32)));

    /// (`Pd`) A dash or hyphen punctuation mark
    pub const DashPunctuation: GeneralCategoryGroup = GCG(1 << (GC::DashPunctuation as u32));
    /// (`Ps`) An opening punctuation mark (of a pair)
    pub const OpenPunctuation: GeneralCategoryGroup = GCG(1 << (GC::OpenPunctuation as u32));
    /// (`Pe`) A closing punctuation mark (of a pair)
    pub const ClosePunctuation: GeneralCategoryGroup = GCG(1 << (GC::ClosePunctuation as u32));
    /// (`Pc`) A connecting punctuation mark, like a tie
    pub const ConnectorPunctuation: GeneralCategoryGroup =
        GCG(1 << (GC::ConnectorPunctuation as u32));
    /// (`Pi`) An initial quotation mark
    pub const InitialPunctuation: GeneralCategoryGroup = GCG(1 << (GC::InitialPunctuation as u32));
    /// (`Pf`) A final quotation mark
    pub const FinalPunctuation: GeneralCategoryGroup = GCG(1 << (GC::FinalPunctuation as u32));
    /// (`Po`) A punctuation mark of other type
    pub const OtherPunctuation: GeneralCategoryGroup = GCG(1 << (GC::OtherPunctuation as u32));
    /// (`P`) The union of all punctuation categories
    pub const Punctuation: GeneralCategoryGroup = GCG((1 << (GC::DashPunctuation as u32))
        | (1 << (GC::OpenPunctuation as u32))
        | (1 << (GC::ClosePunctuation as u32))
        | (1 << (GC::ConnectorPunctuation as u32))
        | (1 << (GC::OtherPunctuation as u32))
        | (1 << (GC::InitialPunctuation as u32))
        | (1 << (GC::FinalPunctuation as u32)));

    /// (`Sm`) A symbol of mathematical use
    pub const MathSymbol: GeneralCategoryGroup = GCG(1 << (GC::MathSymbol as u32));
    /// (`Sc`) A currency sign
    pub const CurrencySymbol: GeneralCategoryGroup = GCG(1 << (GC::CurrencySymbol as u32));
    /// (`Sk`) A non-letterlike modifier symbol
    pub const ModifierSymbol: GeneralCategoryGroup = GCG(1 << (GC::ModifierSymbol as u32));
    /// (`So`) A symbol of other type
    pub const OtherSymbol: GeneralCategoryGroup = GCG(1 << (GC::OtherSymbol as u32));
    /// (`S`) The union of all symbol categories
    pub const Symbol: GeneralCategoryGroup = GCG((1 << (GC::MathSymbol as u32))
        | (1 << (GC::CurrencySymbol as u32))
        | (1 << (GC::ModifierSymbol as u32))
        | (1 << (GC::OtherSymbol as u32)));

    const ALL: u32 = (1 << (GC::FinalPunctuation as u32 + 1)) - 1;

    #[cfg(feature = "datagen")]
    #[doc(hidden)]
    pub fn names() -> impl Iterator<Item = (&'static str, Self)> {
        [
            ("Lu", Self::UppercaseLetter),
            ("Ll", Self::LowercaseLetter),
            ("Lt", Self::TitlecaseLetter),
            ("Lm", Self::ModifierLetter),
            ("Lo", Self::OtherLetter),
            ("LC", Self::CasedLetter),
            ("L", Self::Letter),
            ("Mn", Self::NonspacingMark),
            ("Me", Self::EnclosingMark),
            ("Mc", Self::SpacingMark),
            ("M", Self::Mark),
            ("Nd", Self::DecimalNumber),
            ("Nl", Self::LetterNumber),
            ("No", Self::OtherNumber),
            ("N", Self::Number),
            ("Zs", Self::SpaceSeparator),
            ("Zl", Self::LineSeparator),
            ("Zp", Self::ParagraphSeparator),
            ("Z", Self::Separator),
            ("Cc", Self::Control),
            ("Cf", Self::Format),
            ("Co", Self::PrivateUse),
            ("Cs", Self::Surrogate),
            ("Cn", Self::Unassigned),
            ("C", Self::Other),
            ("Pd", Self::DashPunctuation),
            ("Ps", Self::OpenPunctuation),
            ("Pe", Self::ClosePunctuation),
            ("Pc", Self::ConnectorPunctuation),
            ("Pi", Self::InitialPunctuation),
            ("Pf", Self::FinalPunctuation),
            ("Po", Self::OtherPunctuation),
            ("P", Self::Punctuation),
            ("Sm", Self::MathSymbol),
            ("Sc", Self::CurrencySymbol),
            ("Sk", Self::ModifierSymbol),
            ("So", Self::OtherSymbol),
            ("S", Self::Symbol),
        ]
        .into_iter()
    }

    /// Return whether the code point belongs in the provided multi-value category.
    ///
    /// ```
    /// use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
    /// use icu::properties::CodePointMapData;
    ///
    /// let gc = CodePointMapData::<GeneralCategory>::new();
    ///
    /// assert_eq!(gc.get('A'), GeneralCategory::UppercaseLetter);
    /// assert!(GeneralCategoryGroup::CasedLetter.contains(gc.get('A')));
    ///
    /// // U+0B1E ORIYA LETTER NYA
    /// assert_eq!(gc.get('ଞ'), GeneralCategory::OtherLetter);
    /// assert!(GeneralCategoryGroup::Letter.contains(gc.get('ଞ')));
    /// assert!(!GeneralCategoryGroup::CasedLetter.contains(gc.get('ଞ')));
    ///
    /// // U+0301 COMBINING ACUTE ACCENT
    /// assert_eq!(gc.get('\u{0301}'), GeneralCategory::NonspacingMark);
    /// assert!(GeneralCategoryGroup::Mark.contains(gc.get('\u{0301}')));
    /// assert!(!GeneralCategoryGroup::Letter.contains(gc.get('\u{0301}')));
    ///
    /// assert_eq!(gc.get('0'), GeneralCategory::DecimalNumber);
    /// assert!(GeneralCategoryGroup::Number.contains(gc.get('0')));
    /// assert!(!GeneralCategoryGroup::Mark.contains(gc.get('0')));
    ///
    /// assert_eq!(gc.get('('), GeneralCategory::OpenPunctuation);
    /// assert!(GeneralCategoryGroup::Punctuation.contains(gc.get('(')));
    /// assert!(!GeneralCategoryGroup::Number.contains(gc.get('(')));
    ///
    /// // U+2713 CHECK MARK
    /// assert_eq!(gc.get('✓'), GeneralCategory::OtherSymbol);
    /// assert!(GeneralCategoryGroup::Symbol.contains(gc.get('✓')));
    /// assert!(!GeneralCategoryGroup::Punctuation.contains(gc.get('✓')));
    ///
    /// assert_eq!(gc.get(' '), GeneralCategory::SpaceSeparator);
    /// assert!(GeneralCategoryGroup::Separator.contains(gc.get(' ')));
    /// assert!(!GeneralCategoryGroup::Symbol.contains(gc.get(' ')));
    ///
    /// // U+E007F CANCEL TAG
    /// assert_eq!(gc.get('\u{E007F}'), GeneralCategory::Format);
    /// assert!(GeneralCategoryGroup::Other.contains(gc.get('\u{E007F}')));
    /// assert!(!GeneralCategoryGroup::Separator.contains(gc.get('\u{E007F}')));
    /// ```
    pub const fn contains(self, val: GeneralCategory) -> bool {
        0 != (1 << (val as u32)) & self.0
    }

    /// Produce a `GeneralCategoryGroup` that is the inverse of this one
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
    ///
    /// let letter = GeneralCategoryGroup::Letter;
    /// let not_letter = letter.complement();
    ///
    /// assert!(not_letter.contains(GeneralCategory::MathSymbol));
    /// assert!(!letter.contains(GeneralCategory::MathSymbol));
    /// assert!(not_letter.contains(GeneralCategory::OtherPunctuation));
    /// assert!(!letter.contains(GeneralCategory::OtherPunctuation));
    /// assert!(!not_letter.contains(GeneralCategory::UppercaseLetter));
    /// assert!(letter.contains(GeneralCategory::UppercaseLetter));
    /// ```
    pub const fn complement(self) -> Self {
        // Mask off things not in Self::ALL to guarantee the mask
        // values stay in-range
        GeneralCategoryGroup(!self.0 & Self::ALL)
    }

    /// Return the group representing all `GeneralCategory` values
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
    ///
    /// let all = GeneralCategoryGroup::all();
    ///
    /// assert!(all.contains(GeneralCategory::MathSymbol));
    /// assert!(all.contains(GeneralCategory::OtherPunctuation));
    /// assert!(all.contains(GeneralCategory::UppercaseLetter));
    /// ```
    pub const fn all() -> Self {
        Self(Self::ALL)
    }

    /// Return the empty group
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
    ///
    /// let empty = GeneralCategoryGroup::empty();
    ///
    /// assert!(!empty.contains(GeneralCategory::MathSymbol));
    /// assert!(!empty.contains(GeneralCategory::OtherPunctuation));
    /// assert!(!empty.contains(GeneralCategory::UppercaseLetter));
    /// ```
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Take the union of two groups
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
    ///
    /// let letter = GeneralCategoryGroup::Letter;
    /// let symbol = GeneralCategoryGroup::Symbol;
    /// let union = letter.union(symbol);
    ///
    /// assert!(union.contains(GeneralCategory::MathSymbol));
    /// assert!(!union.contains(GeneralCategory::OtherPunctuation));
    /// assert!(union.contains(GeneralCategory::UppercaseLetter));
    /// ```
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Take the intersection of two groups
    ///
    /// # Example
    ///
    /// ```rust
    /// use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
    ///
    /// let letter = GeneralCategoryGroup::Letter;
    /// let lu = GeneralCategoryGroup::UppercaseLetter;
    /// let intersection = letter.intersection(lu);
    ///
    /// assert!(!intersection.contains(GeneralCategory::MathSymbol));
    /// assert!(!intersection.contains(GeneralCategory::OtherPunctuation));
    /// assert!(intersection.contains(GeneralCategory::UppercaseLetter));
    /// assert!(!intersection.contains(GeneralCategory::LowercaseLetter));
    /// ```
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}

impl From<GeneralCategory> for GeneralCategoryGroup {
    fn from(subcategory: GeneralCategory) -> Self {
        GeneralCategoryGroup(1 << (subcategory as u32))
    }
}
impl From<u32> for GeneralCategoryGroup {
    fn from(mask: u32) -> Self {
        // Mask off things not in Self::ALL to guarantee the mask
        // values stay in-range
        GeneralCategoryGroup(mask & Self::ALL)
    }
}
impl From<GeneralCategoryGroup> for u32 {
    fn from(group: GeneralCategoryGroup) -> Self {
        group.0
    }
}

/// Enumerated property Script.
///
/// This is used with both the Script and `Script_Extensions` Unicode properties.
/// Each character is assigned a single Script, but characters that are used in
/// a particular subset of scripts will be in more than one `Script_Extensions` set.
/// For example, `DEVANAGARI DIGIT NINE` has `Script=Devanagari`, but is also in the
/// `Script_Extensions` set for Dogra, Kaithi, and Mahajani. If you are trying to
/// determine whether a code point belongs to a certain script, you should use
/// [`ScriptWithExtensionsBorrowed::has_script`].
///
/// For more information, see UAX #24: <https://www.unicode.org/reports/tr24/>.
/// See `UScriptCode` in ICU4C.
///
/// # Example
///
/// ```
/// use icu::properties::{CodePointMapData, props::Script};
///
/// assert_eq!(CodePointMapData::<Script>::new().get('木'), Script::Han);  // U+6728
/// assert_eq!(CodePointMapData::<Script>::new().get('🎃'), Script::Common);  // U+1F383 JACK-O-LANTERN
/// ```
/// [`ScriptWithExtensionsBorrowed::has_script`]: crate::script::ScriptWithExtensionsBorrowed::has_script
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct Script(pub(crate) u16);

impl Script {
    /// Returns an ICU4C `UScriptCode` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u16 {
        self.0
    }
    /// Constructor from an ICU4C `UScriptCode` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u16) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl Script {
    #[default]
    pub const Unknown: Script = Script(103); / "Zzzz"
    pub const Adlam: Script = Script(167); / "Adlm"
    pub const Ahom: Script = Script(161); / "Ahom"
    pub const AnatolianHieroglyphs: Script = Script(156); / "Hluw"
    pub const Arabic: Script = Script(2); / "Arab"
    pub const Armenian: Script = Script(3); / "Armn"
    pub const Avestan: Script = Script(117); / "Avst"
    pub const Balinese: Script = Script(62); / "Bali"
    pub const Bamum: Script = Script(130); / "Bamu"
    pub const BassaVah: Script = Script(134); / "Bass"
    pub const Batak: Script = Script(63); / "Batk"
    pub const Bengali: Script = Script(4); / "Beng"
    pub const BeriaErfe: Script = Script(208); / "Berf"
    pub const Bhaiksuki: Script = Script(168); / "Bhks"
    pub const Bopomofo: Script = Script(5); / "Bopo"
    pub const Brahmi: Script = Script(65); / "Brah"
    pub const Braille: Script = Script(46); / "Brai"
    pub const Buginese: Script = Script(55); / "Bugi"
    pub const Buhid: Script = Script(44); / "Buhd"
    pub const CanadianAboriginal: Script = Script(40); / "Cans"
    pub const Carian: Script = Script(104); / "Cari"
    pub const CaucasianAlbanian: Script = Script(159); / "Aghb"
    pub const Chakma: Script = Script(118); / "Cakm"
    pub const Cham: Script = Script(66); / "Cham"
    pub const Cherokee: Script = Script(6); / "Cher"
    pub const Chorasmian: Script = Script(189); / "Chrs"
    pub const Common: Script = Script(0); / "Zyyy"
    pub const Coptic: Script = Script(7); / "Copt"
    pub const Cuneiform: Script = Script(101); / "Xsux"
    pub const Cypriot: Script = Script(47); / "Cprt"
    pub const CyproMinoan: Script = Script(193); / "Cpmn"
    pub const Cyrillic: Script = Script(8); / "Cyrl"
    pub const Deseret: Script = Script(9); / "Dsrt"
    pub const Devanagari: Script = Script(10); / "Deva"
    pub const DivesAkuru: Script = Script(190); / "Diak"
    pub const Dogra: Script = Script(178); / "Dogr"
    pub const Duployan: Script = Script(135); / "Dupl"
    pub const EgyptianHieroglyphs: Script = Script(71); / "Egyp"
    pub const Elbasan: Script = Script(136); / "Elba"
    pub const Elymaic: Script = Script(185); / "Elym"
    pub const Ethiopian: Script = Script(11); / "Ethi"
    pub const Garay: Script = Script(201); / "Gara"
    pub const Georgian: Script = Script(12); / "Geor"
    pub const Glagolitic: Script = Script(56); / "Glag"
    pub const Gothic: Script = Script(13); / "Goth"
    pub const Grantha: Script = Script(137); / "Gran"
    pub const Greek: Script = Script(14); / "Grek"
    pub const Gujarati: Script = Script(15); / "Gujr"
    pub const GunjalaGondi: Script = Script(179); / "Gong"
    pub const Gurmukhi: Script = Script(16); / "Guru"
    pub const GurungKhema: Script = Script(202); / "Gukh"
    pub const Han: Script = Script(17); / "Hani"
    pub const Hangul: Script = Script(18); / "Hang"
    pub const HanifiRohingya: Script = Script(182); / "Rohg"
    pub const Hanunoo: Script = Script(43); / "Hano"
    pub const Hatran: Script = Script(162); / "Hatr"
    pub const Hebrew: Script = Script(19); / "Hebr"
    pub const Hiragana: Script = Script(20); / "Hira"
    pub const ImperialAramaic: Script = Script(116); / "Armi"
    pub const Inherited: Script = Script(1); / "Zinh"
    pub const InscriptionalPahlavi: Script = Script(122); / "Phli"
    pub const InscriptionalParthian: Script = Script(125); / "Prti"
    pub const Javanese: Script = Script(78); / "Java"
    pub const Kaithi: Script = Script(120); / "Kthi"
    pub const Kannada: Script = Script(21); / "Knda"
    pub const Katakana: Script = Script(22); / "Kana"
    pub const Kawi: Script = Script(198); / "Kawi"
    pub const KayahLi: Script = Script(79); / "Kali"
    pub const Kharoshthi: Script = Script(57); / "Khar"
    pub const KhitanSmallScript: Script = Script(191); / "Kits"
    pub const Khmer: Script = Script(23); / "Khmr"
    pub const Khojki: Script = Script(157); / "Khoj"
    pub const Khudawadi: Script = Script(145); / "Sind"
    pub const KiratRai: Script = Script(203); / "Krai"
    pub const Lao: Script = Script(24); / "Laoo"
    pub const Latin: Script = Script(25); / "Latn"
    pub const Lepcha: Script = Script(82); / "Lepc"
    pub const Limbu: Script = Script(48); / "Limb"
    pub const LinearA: Script = Script(83); / "Lina"
    pub const LinearB: Script = Script(49); / "Linb"
    pub const Lisu: Script = Script(131); / "Lisu"
    pub const Lycian: Script = Script(107); / "Lyci"
    pub const Lydian: Script = Script(108); / "Lydi"
    pub const Mahajani: Script = Script(160); / "Mahj"
    pub const Makasar: Script = Script(180); / "Maka"
    pub const Malayalam: Script = Script(26); / "Mlym"
    pub const Mandaic: Script = Script(84); / "Mand"
    pub const Manichaean: Script = Script(121); / "Mani"
    pub const Marchen: Script = Script(169); / "Marc"
    pub const MasaramGondi: Script = Script(175); / "Gonm"
    pub const Medefaidrin: Script = Script(181); / "Medf"
    pub const MeeteiMayek: Script = Script(115); / "Mtei"
    pub const MendeKikakui: Script = Script(140); / "Mend"
    pub const MeroiticCursive: Script = Script(141); / "Merc"
    pub const MeroiticHieroglyphs: Script = Script(86); / "Mero"
    pub const Miao: Script = Script(92); / "Plrd"
    pub const Modi: Script = Script(163); / "Modi"
    pub const Mongolian: Script = Script(27); / "Mong"
    pub const Mro: Script = Script(149); / "Mroo"
    pub const Multani: Script = Script(164); / "Mult"
    pub const Myanmar: Script = Script(28); / "Mymr"
    pub const Nabataean: Script = Script(143); / "Nbat"
    pub const NagMundari: Script = Script(199); / "Nagm"
    pub const Nandinagari: Script = Script(187); / "Nand"
    pub const Newa: Script = Script(170); / "Newa"
    pub const NewTaiLue: Script = Script(59); / "Talu"
    pub const Nko: Script = Script(87); / "Nkoo"
    pub const Nushu: Script = Script(150); / "Nshu"
    pub const NyiakengPuachueHmong: Script = Script(186); / "Hmnp"
    pub const Ogham: Script = Script(29); / "Ogam"
    pub const OlChiki: Script = Script(109); / "Olck"
    pub const OldHungarian: Script = Script(76); / "Hung"
    pub const OldItalic: Script = Script(30); / "Ital"
    pub const OldNorthArabian: Script = Script(142); / "Narb"
    pub const OldPermic: Script = Script(89); / "Perm"
    pub const OldPersian: Script = Script(61); / "Xpeo"
    pub const OldSogdian: Script = Script(184); / "Sogo"
    pub const OldSouthArabian: Script = Script(133); / "Sarb"
    pub const OldTurkic: Script = Script(88); / "Orkh"
    pub const OldUyghur: Script = Script(194); / "Ougr"
    pub const OlOnal: Script = Script(204); / "Onao"
    pub const Oriya: Script = Script(31); / "Orya"
    pub const Osage: Script = Script(171); / "Osge"
    pub const Osmanya: Script = Script(50); / "Osma"
    pub const PahawhHmong: Script = Script(75); / "Hmng"
    pub const Palmyrene: Script = Script(144); / "Palm"
    pub const PauCinHau: Script = Script(165); / "Pauc"
    pub const PhagsPa: Script = Script(90); / "Phag"
    pub const Phoenician: Script = Script(91); / "Phnx"
    pub const PsalterPahlavi: Script = Script(123); / "Phlp"
    pub const Rejang: Script = Script(110); / "Rjng"
    pub const Runic: Script = Script(32); / "Runr"
    pub const Samaritan: Script = Script(126); / "Samr"
    pub const Saurashtra: Script = Script(111); / "Saur"
    pub const Sharada: Script = Script(151); / "Shrd"
    pub const Shavian: Script = Script(51); / "Shaw"
    pub const Siddham: Script = Script(166); / "Sidd"
    pub const Sidetic: Script = Script(209); / "Sidt"
    pub const SignWriting: Script = Script(112); / "Sgnw"
    pub const Sinhala: Script = Script(33); / "Sinh"
    pub const Sogdian: Script = Script(183); / "Sogd"
    pub const SoraSompeng: Script = Script(152); / "Sora"
    pub const Soyombo: Script = Script(176); / "Soyo"
    pub const Sundanese: Script = Script(113); / "Sund"
    pub const Sunuwar: Script = Script(205); / "Sunu"
    pub const SylotiNagri: Script = Script(58); / "Sylo"
    pub const Syriac: Script = Script(34); / "Syrc"
    pub const Tagalog: Script = Script(42); / "Tglg"
    pub const Tagbanwa: Script = Script(45); / "Tagb"
    pub const TaiLe: Script = Script(52); / "Tale"
    pub const TaiTham: Script = Script(106); / "Lana"
    pub const TaiViet: Script = Script(127); / "Tavt"
    pub const TaiYo: Script = Script(210); / "Tayo"
    pub const Takri: Script = Script(153); / "Takr"
    pub const Tamil: Script = Script(35); / "Taml"
    pub const Tangsa: Script = Script(195); / "Tnsa"
    pub const Tangut: Script = Script(154); / "Tang"
    pub const Telugu: Script = Script(36); / "Telu"
    pub const Thaana: Script = Script(37); / "Thaa"
    pub const Thai: Script = Script(38); / "Thai"
    pub const Tibetan: Script = Script(39); / "Tibt"
    pub const Tifinagh: Script = Script(60); / "Tfng"
    pub const Tirhuta: Script = Script(158); / "Tirh"
    pub const Todhri: Script = Script(206); / "Todr"
    pub const TolongSiki: Script = Script(211); / "Tols"
    pub const Toto: Script = Script(196); / "Toto"
    pub const TuluTigalari: Script = Script(207); / "Tutg"
    pub const Ugaritic: Script = Script(53); / "Ugar"
    pub const Vai: Script = Script(99); / "Vaii"
    pub const Vithkuqi: Script = Script(197); / "Vith"
    pub const Wancho: Script = Script(188); / "Wcho"
    pub const WarangCiti: Script = Script(146); / "Wara"
    pub const Yezidi: Script = Script(192); / "Yezi"
    pub const Yi: Script = Script(41); / "Yiii"
    pub const ZanabazarSquare: Script = Script(177); / "Zanb"

    // These 38 scripts are scripts that ICU defines but Unicode
    // doesn't. We define them for parity with ICU, and so
    // that our short name map can be dense.
    // It's unclear why we have exposed an identifier for
    // Nastaliq but not for any of the others.
    pub const Nastaliq: Script = Script(200); / "Aran"
    "Afak" => 147;
    "Blis" => 64;
    "Cirt" => 67;
    "Cyrs" => 68;
    "Egyd" => 69;
    "Egyh" => 70;
    "Geok" => 72;
    "Hanb" => 172;
    "Hans" => 73;
    "Hant" => 74;
    "Hntl" => 212;
    "Hrkt" => 54;
    "Inds" => 77;
    "Jamo" => 173;
    "Jpan" => 105;
    "Jurc" => 148;
    "Kore" => 119;
    "Kpel" => 138;
    "Latf" => 80;
    "Latg" => 81;
    "Loma" => 139;
    "Maya" => 85;
    "Moon" => 114;
    "Nkgb" => 132;
    "Phlv" => 124;
    "Roro" => 93;
    "Sara" => 94;
    "Syre" => 95;
    "Syrj" => 96;
    "Syrn" => 97;
    "Teng" => 98;
    "Visp" => 100;
    "Wole" => 155;
    "Zmth" => 128;
    "Zsye" => 174;
    "Zsym" => 129;
    "Zxxx" => 102;
}
#[test]
fn script_consts();
}

impl Script {
    // Doesn't actually exist!
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    #[deprecated]
    // Some high value that ICU4C will not use anytime soon
    pub const Chisoi: Script = Self(60_000);
}

/// ✨ *Enabled with the `compiled_data` Cargo feature.*
#[cfg(feature = "compiled_data")]
impl From<Script> for icu_locale_core::subtags::Script {
    fn from(value: Script) -> Self {
        crate::PropertyNamesShort::new()
            .get_locale_script(value)
            .unwrap_or(icu_locale_core::subtags::script!("Zzzz"))
    }
}

/// ✨ *Enabled with the `compiled_data` Cargo feature.*
#[cfg(feature = "compiled_data")]
impl From<icu_locale_core::subtags::Script> for Script {
    fn from(value: icu_locale_core::subtags::Script) -> Self {
        crate::PropertyParser::new()
            .get_strict(value.as_str())
            .unwrap_or(Self::Unknown)
    }
}

make_enumerated_property! {
    name: "Script";
    short_name: "sc";
    ident: Script;
    data_marker: crate::provider::PropertyEnumScriptV1;
    singleton: SINGLETON_PROPERTY_ENUM_SCRIPT_V1;
    ule_ty: <u16 as zerovec::ule::AsULE>::ULE;
}

/// Enumerated property `Hangul_Syllable_Type`
///
/// The Unicode standard provides both precomposed Hangul syllables and conjoining Jamo to compose
/// arbitrary Hangul syllables. This property provides that ontology of Hangul code points.
///
/// For more information, see the [Unicode Korean FAQ](https://www.unicode.org/faq/korean.html).
///
/// # Example
///
/// ```
/// use icu::properties::{props::HangulSyllableType, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<HangulSyllableType>::new().get('ᄀ'),
///     HangulSyllableType::LeadingJamo
/// ); // U+1100
/// assert_eq!(
///     CodePointMapData::<HangulSyllableType>::new().get('가'),
///     HangulSyllableType::LeadingVowelSyllable
/// ); // U+AC00
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct HangulSyllableType(pub(crate) u8);

impl HangulSyllableType {
    /// Returns an ICU4C `UHangulSyllableType` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UHangulSyllableType` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(non_upper_case_globals)]
impl HangulSyllableType {
    #[default]
    /// (`NA`) not applicable (e.g. not a Hangul code point).
    pub const NotApplicable: HangulSyllableType = HangulSyllableType(0); / "NA"
    /// (`L`) a conjoining leading consonant Jamo.
    pub const LeadingJamo: HangulSyllableType = HangulSyllableType(1); / "L"
    /// (`V`) a conjoining vowel Jamo.
    pub const VowelJamo: HangulSyllableType = HangulSyllableType(2); / "V"
    /// (`T`) a conjoining trailing consonant Jamo.
    pub const TrailingJamo: HangulSyllableType = HangulSyllableType(3); / "T"
    /// (`LV`) a precomposed syllable with a leading consonant and a vowel.
    pub const LeadingVowelSyllable: HangulSyllableType = HangulSyllableType(4); / "LV"
    /// (`LVT`) a precomposed syllable with a leading consonant, a vowel, and a trailing consonant.
    pub const LeadingVowelTrailingSyllable: HangulSyllableType = HangulSyllableType(5); / "LVT"
}
#[test]
fn hangul_syllable_type_consts();
}

make_enumerated_property! {
    name: "Hangul_Syllable_Type";
    short_name: "hst";
    ident: HangulSyllableType;
    data_marker: crate::provider::PropertyEnumHangulSyllableTypeV1;
    singleton: SINGLETON_PROPERTY_ENUM_HANGUL_SYLLABLE_TYPE_V1;
    ule_ty: u8;

}

/// Enumerated property `East_Asian_Width`.
///
/// See "Definition" in UAX #11 for the summary of each property value:
/// <https://www.unicode.org/reports/tr11/#Definitions>
///
/// # Example
///
/// ```
/// use icu::properties::{props::EastAsianWidth, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<EastAsianWidth>::new().get('ｱ'),
///     EastAsianWidth::Halfwidth
/// ); // U+FF71: Halfwidth Katakana Letter A
/// assert_eq!(
///     CodePointMapData::<EastAsianWidth>::new().get('ア'),
///     EastAsianWidth::Wide
/// ); //U+30A2: Katakana Letter A
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct EastAsianWidth(pub(crate) u8);

impl EastAsianWidth {
    /// Returns an ICU4C `UEastAsianWidth` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UEastAsianWidth` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl EastAsianWidth {
    #[default]
    pub const Neutral: EastAsianWidth = EastAsianWidth(0); / "N"
    pub const Ambiguous: EastAsianWidth = EastAsianWidth(1); / "A"
    pub const Halfwidth: EastAsianWidth = EastAsianWidth(2); / "H"
    pub const Fullwidth: EastAsianWidth = EastAsianWidth(3); / "F"
    pub const Narrow: EastAsianWidth = EastAsianWidth(4); / "Na"
    pub const Wide: EastAsianWidth = EastAsianWidth(5); / "W"
}
#[test]
fn east_asian_width_consts();
}

make_enumerated_property! {
    name: "East_Asian_Width";
    short_name: "ea";
    ident: EastAsianWidth;
    data_marker: crate::provider::PropertyEnumEastAsianWidthV1;
    singleton: SINGLETON_PROPERTY_ENUM_EAST_ASIAN_WIDTH_V1;
    ule_ty: u8;
}

/// Enumerated property `Line_Break`.
///
/// See "Line Breaking Properties" in UAX #14 for the summary of each property
/// value: <https://www.unicode.org/reports/tr14/#Properties>
///
/// The numeric value is compatible with `ULineBreak` in ICU4C.
///
/// **Note:** Use `icu::segmenter` for an all-in-one break iterator implementation.
///
/// # Example
///
/// ```
/// use icu::properties::{props::LineBreak, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<LineBreak>::new().get(')'),
///     LineBreak::CloseParenthesis
/// ); // U+0029: Right Parenthesis
/// assert_eq!(
///     CodePointMapData::<LineBreak>::new().get('ぁ'),
///     LineBreak::ConditionalJapaneseStarter
/// ); //U+3041: Hiragana Letter Small A
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct LineBreak(pub(crate) u8);

impl LineBreak {
    /// Returns an ICU4C `ULineBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `ULineBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl LineBreak {
    #[default]
    pub const Unknown: LineBreak = LineBreak(0); / "XX"
    pub const Ambiguous: LineBreak = LineBreak(1); / "AI"
    pub const Alphabetic: LineBreak = LineBreak(2); / "AL"
    pub const BreakBoth: LineBreak = LineBreak(3); / "B2"
    pub const BreakAfter: LineBreak = LineBreak(4); / "BA"
    pub const BreakBefore: LineBreak = LineBreak(5); / "BB"
    pub const MandatoryBreak: LineBreak = LineBreak(6); / "BK"
    pub const ContingentBreak: LineBreak = LineBreak(7); / "CB"
    pub const ClosePunctuation: LineBreak = LineBreak(8); / "CL"
    pub const CombiningMark: LineBreak = LineBreak(9); / "CM"
    pub const CarriageReturn: LineBreak = LineBreak(10); / "CR"
    pub const Exclamation: LineBreak = LineBreak(11); / "EX"
    pub const Glue: LineBreak = LineBreak(12); / "GL"
    pub const Hyphen: LineBreak = LineBreak(13); / "HY"
    pub const Ideographic: LineBreak = LineBreak(14); / "ID"
    pub const Inseparable: LineBreak = LineBreak(15); / "IN"
    pub const InfixNumeric: LineBreak = LineBreak(16); / "IS"
    pub const LineFeed: LineBreak = LineBreak(17); / "LF"
    pub const Nonstarter: LineBreak = LineBreak(18); / "NS"
    pub const Numeric: LineBreak = LineBreak(19); / "NU"
    pub const OpenPunctuation: LineBreak = LineBreak(20); / "OP"
    pub const PostfixNumeric: LineBreak = LineBreak(21); / "PO"
    pub const PrefixNumeric: LineBreak = LineBreak(22); / "PR"
    pub const Quotation: LineBreak = LineBreak(23); / "QU"
    pub const ComplexContext: LineBreak = LineBreak(24); / "SA"
    pub const Surrogate: LineBreak = LineBreak(25); / "SG"
    pub const Space: LineBreak = LineBreak(26); / "SP"
    pub const BreakSymbols: LineBreak = LineBreak(27); / "SY"
    pub const ZWSpace: LineBreak = LineBreak(28); / "ZW"
    pub const NextLine: LineBreak = LineBreak(29); / "NL"
    pub const WordJoiner: LineBreak = LineBreak(30); / "WJ"
    pub const H2: LineBreak = LineBreak(31); / "H2"
    pub const H3: LineBreak = LineBreak(32); / "H3"
    pub const JL: LineBreak = LineBreak(33); / "JL"
    pub const JT: LineBreak = LineBreak(34); / "JT"
    pub const JV: LineBreak = LineBreak(35); / "JV"
    pub const CloseParenthesis: LineBreak = LineBreak(36); / "CP"
    pub const ConditionalJapaneseStarter: LineBreak = LineBreak(37); / "CJ"
    pub const HebrewLetter: LineBreak = LineBreak(38); / "HL"
    pub const RegionalIndicator: LineBreak = LineBreak(39); / "RI"
    pub const EBase: LineBreak = LineBreak(40); / "EB"
    pub const EModifier: LineBreak = LineBreak(41); / "EM"
    pub const ZWJ: LineBreak = LineBreak(42); / "ZWJ"

    // Added in Unicode 15.1:
    pub const Aksara: LineBreak = LineBreak(43); / "AK"
    pub const AksaraPrebase: LineBreak = LineBreak(44); / "AP"
    pub const AksaraStart: LineBreak = LineBreak(45); / "AS"
    pub const ViramaFinal: LineBreak = LineBreak(46); / "VF"
    pub const Virama: LineBreak = LineBreak(47); / "VI"

    // Added in Unicode 17:
    pub const UnambiguousHyphen: LineBreak = LineBreak(48); / "HH"
}
#[test]
fn line_break_consts();
}

make_enumerated_property! {
    name: "Line_Break";
    short_name: "lb";
    ident: LineBreak;
    data_marker: crate::provider::PropertyEnumLineBreakV1;
    singleton: SINGLETON_PROPERTY_ENUM_LINE_BREAK_V1;
    ule_ty: u8;
}

/// Enumerated property `Grapheme_Cluster_Break`.
///
/// See "Default Grapheme Cluster Boundary Specification" in UAX #29 for the
/// summary of each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Grapheme_Cluster_Table>
///
/// **Note:** Use `icu::segmenter` for an all-in-one break iterator implementation.
///
/// # Example
///
/// ```
/// use icu::properties::{props::GraphemeClusterBreak, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<GraphemeClusterBreak>::new().get('🇦'),
///     GraphemeClusterBreak::RegionalIndicator
/// ); // U+1F1E6: Regional Indicator Symbol Letter A
/// assert_eq!(
///     CodePointMapData::<GraphemeClusterBreak>::new().get('ำ'),
///     GraphemeClusterBreak::SpacingMark
/// ); //U+0E33: Thai Character Sara Am
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // this type is stable
#[repr(transparent)]
pub struct GraphemeClusterBreak(pub(crate) u8);

impl GraphemeClusterBreak {
    /// Returns an ICU4C `UGraphemeClusterBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UGraphemeClusterBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl GraphemeClusterBreak {
    #[default]
    pub const Other: GraphemeClusterBreak = GraphemeClusterBreak(0); / "XX"
    pub const Control: GraphemeClusterBreak = GraphemeClusterBreak(1); / "CN"
    pub const CR: GraphemeClusterBreak = GraphemeClusterBreak(2); / "CR"
    pub const Extend: GraphemeClusterBreak = GraphemeClusterBreak(3); / "EX"
    pub const L: GraphemeClusterBreak = GraphemeClusterBreak(4); / "L"
    pub const LF: GraphemeClusterBreak = GraphemeClusterBreak(5); / "LF"
    pub const LV: GraphemeClusterBreak = GraphemeClusterBreak(6); / "LV"
    pub const LVT: GraphemeClusterBreak = GraphemeClusterBreak(7); / "LVT"
    pub const T: GraphemeClusterBreak = GraphemeClusterBreak(8); / "T"
    pub const V: GraphemeClusterBreak = GraphemeClusterBreak(9); / "V"
    pub const SpacingMark: GraphemeClusterBreak = GraphemeClusterBreak(10); / "SM"
    pub const Prepend: GraphemeClusterBreak = GraphemeClusterBreak(11); / "PP"
    pub const RegionalIndicator: GraphemeClusterBreak = GraphemeClusterBreak(12); / "RI"
    /// This value is obsolete and unused.
    pub const EBase: GraphemeClusterBreak = GraphemeClusterBreak(13); / "EB"
    /// This value is obsolete and unused.
    pub const EBaseGAZ: GraphemeClusterBreak = GraphemeClusterBreak(14); / "EBG"
    /// This value is obsolete and unused.
    pub const EModifier: GraphemeClusterBreak = GraphemeClusterBreak(15); / "EM"
    /// This value is obsolete and unused.
    pub const GlueAfterZwj: GraphemeClusterBreak = GraphemeClusterBreak(16); / "GAZ"
    pub const ZWJ: GraphemeClusterBreak = GraphemeClusterBreak(17); / "ZWJ"
}
#[test]
fn gcb_consts();
}

make_enumerated_property! {
    name: "Grapheme_Cluster_Break";
    short_name: "GCB";
    ident: GraphemeClusterBreak;
    data_marker: crate::provider::PropertyEnumGraphemeClusterBreakV1;
    singleton: SINGLETON_PROPERTY_ENUM_GRAPHEME_CLUSTER_BREAK_V1;
    ule_ty: u8;
}

/// Enumerated property `Word_Break`.
///
/// See "Default Word Boundary Specification" in UAX #29 for the summary of
/// each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>.
///
/// **Note:** Use `icu::segmenter` for an all-in-one break iterator implementation.
///
/// # Example
///
/// ```
/// use icu::properties::{props::WordBreak, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<WordBreak>::new().get('.'),
///     WordBreak::MidNumLet
/// ); // U+002E: Full Stop
/// assert_eq!(
///     CodePointMapData::<WordBreak>::new().get('，'),
///     WordBreak::MidNum
/// ); // U+FF0C: Fullwidth Comma
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct WordBreak(pub(crate) u8);

impl WordBreak {
    /// Returns an ICU4C `UWordBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UWordBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl WordBreak {
    #[default]
    pub const Other: WordBreak = WordBreak(0); / "XX"
    pub const ALetter: WordBreak = WordBreak(1); / "LE"
    pub const Format: WordBreak = WordBreak(2); / "FO"
    pub const Katakana: WordBreak = WordBreak(3); / "KA"
    pub const MidLetter: WordBreak = WordBreak(4); / "ML"
    pub const MidNum: WordBreak = WordBreak(5); / "MN"
    pub const Numeric: WordBreak = WordBreak(6); / "NU"
    pub const ExtendNumLet: WordBreak = WordBreak(7); / "EX"
    pub const CR: WordBreak = WordBreak(8); / "CR"
    pub const Extend: WordBreak = WordBreak(9); / "Extend"
    pub const LF: WordBreak = WordBreak(10); / "LF"
    pub const MidNumLet: WordBreak = WordBreak(11); / "MB"
    pub const Newline: WordBreak = WordBreak(12); / "NL"
    pub const RegionalIndicator: WordBreak = WordBreak(13); / "RI"
    pub const HebrewLetter: WordBreak = WordBreak(14); / "HL"
    pub const SingleQuote: WordBreak = WordBreak(15); / "SQ"
    pub const DoubleQuote: WordBreak = WordBreak(16); / "DQ"
    /// This value is obsolete and unused.
    pub const EBase: WordBreak = WordBreak(17); / "EB"
    /// This value is obsolete and unused.
    pub const EBaseGAZ: WordBreak = WordBreak(18); / "EBG"
    /// This value is obsolete and unused.
    pub const EModifier: WordBreak = WordBreak(19); / "EM"
    /// This value is obsolete and unused.
    pub const GlueAfterZwj: WordBreak = WordBreak(20); / "GAZ"
    pub const ZWJ: WordBreak = WordBreak(21); / "ZWJ"
    pub const WSegSpace: WordBreak = WordBreak(22); / "WSegSpace"
}
#[test]
fn word_break_consts();
}

make_enumerated_property! {
    name: "Word_Break";
    short_name: "WB";
    ident: WordBreak;
    data_marker: crate::provider::PropertyEnumWordBreakV1;
    singleton: SINGLETON_PROPERTY_ENUM_WORD_BREAK_V1;
    ule_ty: u8;
}

/// Enumerated property `Sentence_Break`.
///
/// See "Default Sentence Boundary Specification" in UAX #29 for the summary of
/// each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>.
///
/// **Note:** Use `icu::segmenter` for an all-in-one break iterator implementation.
///
/// # Example
///
/// ```
/// use icu::properties::{props::SentenceBreak, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<SentenceBreak>::new().get('９'),
///     SentenceBreak::Numeric
/// ); // U+FF19: Fullwidth Digit Nine
/// assert_eq!(
///     CodePointMapData::<SentenceBreak>::new().get(','),
///     SentenceBreak::SContinue
/// ); // U+002C: Comma
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct SentenceBreak(pub(crate) u8);

impl SentenceBreak {
    /// Returns an ICU4C `USentenceBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `USentenceBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl SentenceBreak {
    #[default]
    pub const Other: SentenceBreak = SentenceBreak(0); / "XX"
    pub const ATerm: SentenceBreak = SentenceBreak(1); / "AT"
    pub const Close: SentenceBreak = SentenceBreak(2); / "CL"
    pub const Format: SentenceBreak = SentenceBreak(3); / "FO"
    pub const Lower: SentenceBreak = SentenceBreak(4); / "LO"
    pub const Numeric: SentenceBreak = SentenceBreak(5); / "NU"
    pub const OLetter: SentenceBreak = SentenceBreak(6); / "LE"
    pub const Sep: SentenceBreak = SentenceBreak(7); / "SE"
    pub const Sp: SentenceBreak = SentenceBreak(8); / "SP"
    pub const STerm: SentenceBreak = SentenceBreak(9); / "ST"
    pub const Upper: SentenceBreak = SentenceBreak(10); / "UP"
    pub const CR: SentenceBreak = SentenceBreak(11); / "CR"
    pub const Extend: SentenceBreak = SentenceBreak(12); / "EX"
    pub const LF: SentenceBreak = SentenceBreak(13); / "LF"
    pub const SContinue: SentenceBreak = SentenceBreak(14); / "SC"
}
#[test]
fn sentence_break_consts();
}

make_enumerated_property! {
    name: "Sentence_Break";
    short_name: "SB";
    ident: SentenceBreak;
    data_marker: crate::provider::PropertyEnumSentenceBreakV1;
    singleton: SINGLETON_PROPERTY_ENUM_SENTENCE_BREAK_V1;
    ule_ty: u8;
}

/// Property `Canonical_Combining_Class`.
/// See UAX #15:
/// <https://www.unicode.org/reports/tr15/>.
///
/// **Note:** See `icu::normalizer::properties::CanonicalCombiningClassMap` for the preferred API
/// to look up the `Canonical_Combining_Class` property by scalar value.
///
/// # Example
///
/// ```
/// use icu::properties::{props::CanonicalCombiningClass, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<CanonicalCombiningClass>::new().get('a'),
///     CanonicalCombiningClass::NotReordered
/// ); // U+0061: LATIN SMALL LETTER A
/// assert_eq!(
///     CodePointMapData::<CanonicalCombiningClass>::new().get('\u{0301}'),
///     CanonicalCombiningClass::Above
/// ); // U+0301: COMBINING ACUTE ACCENT
/// ```
//
// NOTE: The Pernosco debugger has special knowledge
// of this struct. Please do not change the bit layout
// or the crate-module-qualified name of this struct
// without coordination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct CanonicalCombiningClass(pub u8);

impl CanonicalCombiningClass {
    /// Returns an ICU4C `UCanonicalCombiningClass` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UCanonicalCombiningClass` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
// These constant names come from PropertyValueAliases.txt
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl CanonicalCombiningClass {
    #[default]
    pub const NotReordered: CanonicalCombiningClass = CanonicalCombiningClass(0); / "NR"
    pub const Overlay: CanonicalCombiningClass = CanonicalCombiningClass(1); / "OV"
    pub const HanReading: CanonicalCombiningClass = CanonicalCombiningClass(6); / "HANR"
    pub const Nukta: CanonicalCombiningClass = CanonicalCombiningClass(7); / "NK"
    pub const KanaVoicing: CanonicalCombiningClass = CanonicalCombiningClass(8); / "KV"
    pub const Virama: CanonicalCombiningClass = CanonicalCombiningClass(9); / "VR"
    pub const CCC10: CanonicalCombiningClass = CanonicalCombiningClass(10); / "CCC10"
    pub const CCC11: CanonicalCombiningClass = CanonicalCombiningClass(11); / "CCC11"
    pub const CCC12: CanonicalCombiningClass = CanonicalCombiningClass(12); / "CCC12"
    pub const CCC13: CanonicalCombiningClass = CanonicalCombiningClass(13); / "CCC13"
    pub const CCC14: CanonicalCombiningClass = CanonicalCombiningClass(14); / "CCC14"
    pub const CCC15: CanonicalCombiningClass = CanonicalCombiningClass(15); / "CCC15"
    pub const CCC16: CanonicalCombiningClass = CanonicalCombiningClass(16); / "CCC16"
    pub const CCC17: CanonicalCombiningClass = CanonicalCombiningClass(17); / "CCC17"
    pub const CCC18: CanonicalCombiningClass = CanonicalCombiningClass(18); / "CCC18"
    pub const CCC19: CanonicalCombiningClass = CanonicalCombiningClass(19); / "CCC19"
    pub const CCC20: CanonicalCombiningClass = CanonicalCombiningClass(20); / "CCC20"
    pub const CCC21: CanonicalCombiningClass = CanonicalCombiningClass(21); / "CCC21"
    pub const CCC22: CanonicalCombiningClass = CanonicalCombiningClass(22); / "CCC22"
    pub const CCC23: CanonicalCombiningClass = CanonicalCombiningClass(23); / "CCC23"
    pub const CCC24: CanonicalCombiningClass = CanonicalCombiningClass(24); / "CCC24"
    pub const CCC25: CanonicalCombiningClass = CanonicalCombiningClass(25); / "CCC25"
    pub const CCC26: CanonicalCombiningClass = CanonicalCombiningClass(26); / "CCC26"
    pub const CCC27: CanonicalCombiningClass = CanonicalCombiningClass(27); / "CCC27"
    pub const CCC28: CanonicalCombiningClass = CanonicalCombiningClass(28); / "CCC28"
    pub const CCC29: CanonicalCombiningClass = CanonicalCombiningClass(29); / "CCC29"
    pub const CCC30: CanonicalCombiningClass = CanonicalCombiningClass(30); / "CCC30"
    pub const CCC31: CanonicalCombiningClass = CanonicalCombiningClass(31); / "CCC31"
    pub const CCC32: CanonicalCombiningClass = CanonicalCombiningClass(32); / "CCC32"
    pub const CCC33: CanonicalCombiningClass = CanonicalCombiningClass(33); / "CCC33"
    pub const CCC34: CanonicalCombiningClass = CanonicalCombiningClass(34); / "CCC34"
    pub const CCC35: CanonicalCombiningClass = CanonicalCombiningClass(35); / "CCC35"
    pub const CCC36: CanonicalCombiningClass = CanonicalCombiningClass(36); / "CCC36"
    pub const CCC84: CanonicalCombiningClass = CanonicalCombiningClass(84); / "CCC84"
    pub const CCC91: CanonicalCombiningClass = CanonicalCombiningClass(91); / "CCC91"
    pub const CCC103: CanonicalCombiningClass = CanonicalCombiningClass(103); / "CCC103"
    pub const CCC107: CanonicalCombiningClass = CanonicalCombiningClass(107); / "CCC107"
    pub const CCC118: CanonicalCombiningClass = CanonicalCombiningClass(118); / "CCC118"
    pub const CCC122: CanonicalCombiningClass = CanonicalCombiningClass(122); / "CCC122"
    pub const CCC129: CanonicalCombiningClass = CanonicalCombiningClass(129); / "CCC129"
    pub const CCC130: CanonicalCombiningClass = CanonicalCombiningClass(130); / "CCC130"
    pub const CCC132: CanonicalCombiningClass = CanonicalCombiningClass(132); / "CCC132"
    pub const CCC133: CanonicalCombiningClass = CanonicalCombiningClass(133); / "CCC133" // RESERVED
    pub const AttachedBelowLeft: CanonicalCombiningClass = CanonicalCombiningClass(200); / "ATBL"
    pub const AttachedBelow: CanonicalCombiningClass = CanonicalCombiningClass(202); / "ATB"
    pub const AttachedAbove: CanonicalCombiningClass = CanonicalCombiningClass(214); / "ATA"
    pub const AttachedAboveRight: CanonicalCombiningClass = CanonicalCombiningClass(216); / "ATAR"
    pub const BelowLeft: CanonicalCombiningClass = CanonicalCombiningClass(218); / "BL"
    pub const Below: CanonicalCombiningClass = CanonicalCombiningClass(220); / "B"
    pub const BelowRight: CanonicalCombiningClass = CanonicalCombiningClass(222); / "BR"
    pub const Left: CanonicalCombiningClass = CanonicalCombiningClass(224); / "L"
    pub const Right: CanonicalCombiningClass = CanonicalCombiningClass(226); / "R"
    pub const AboveLeft: CanonicalCombiningClass = CanonicalCombiningClass(228); / "AL"
    pub const Above: CanonicalCombiningClass = CanonicalCombiningClass(230); / "A"
    pub const AboveRight: CanonicalCombiningClass = CanonicalCombiningClass(232); / "AR"
    pub const DoubleBelow: CanonicalCombiningClass = CanonicalCombiningClass(233); / "DB"
    pub const DoubleAbove: CanonicalCombiningClass = CanonicalCombiningClass(234); / "DA"
    pub const IotaSubscript: CanonicalCombiningClass = CanonicalCombiningClass(240); / "IS"
}
#[test]
fn ccc_consts();
}

make_enumerated_property! {
    name: "Canonical_Combining_Class";
    short_name: "ccc";
    ident: CanonicalCombiningClass;
    data_marker: crate::provider::PropertyEnumCanonicalCombiningClassV1;
    singleton: SINGLETON_PROPERTY_ENUM_CANONICAL_COMBINING_CLASS_V1;
    ule_ty: u8;
}

/// Property `Indic_Conjunct_Break`.
/// See UAX #44:
/// <https://www.unicode.org/reports/tr44/#Indic_Conjunct_Break>.
///
/// # Example
///
/// ```
/// use icu::properties::{props::IndicConjunctBreak, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<IndicConjunctBreak>::new().get('a'),
///     IndicConjunctBreak::None
/// );
/// assert_eq!(
///     CodePointMapData::<IndicConjunctBreak>::new().get('\u{094d}'),
///     IndicConjunctBreak::Linker
/// );
/// assert_eq!(
///     CodePointMapData::<IndicConjunctBreak>::new().get('\u{0915}'),
///     IndicConjunctBreak::Consonant
/// );
/// assert_eq!(
///     CodePointMapData::<IndicConjunctBreak>::new().get('\u{0300}'),
///     IndicConjunctBreak::Extend
/// );
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct IndicConjunctBreak(pub(crate) u8);

impl IndicConjunctBreak {
    /// Returns an ICU4C `UIndicConjunctBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UIndicConjunctBreak` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl IndicConjunctBreak {
    #[default]
    pub const None: IndicConjunctBreak = IndicConjunctBreak(0); / "None"
    pub const Consonant: IndicConjunctBreak = IndicConjunctBreak(1); / "Consonant"
    pub const Extend: IndicConjunctBreak = IndicConjunctBreak(2); / "Extend"
    pub const Linker: IndicConjunctBreak = IndicConjunctBreak(3); / "Linker"
}
#[test]
fn indic_conjunct_break_consts();
}

make_enumerated_property! {
    name: "Indic_Conjunct_Break";
    short_name: "InCB";
    ident: IndicConjunctBreak;
    data_marker: crate::provider::PropertyEnumIndicConjunctBreakV1;
    singleton: SINGLETON_PROPERTY_ENUM_INDIC_CONJUNCT_BREAK_V1;
    ule_ty: u8;
}

/// Property `Indic_Syllabic_Category`.
/// See UAX #44:
/// <https://www.unicode.org/reports/tr44/#Indic_Syllabic_Category>.
///
/// # Example
///
/// ```
/// use icu::properties::{props::IndicSyllabicCategory, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<IndicSyllabicCategory>::new().get('a'),
///     IndicSyllabicCategory::Other
/// );
/// assert_eq!(
///     CodePointMapData::<IndicSyllabicCategory>::new().get('\u{0900}'),
///     IndicSyllabicCategory::Bindu
/// ); // U+0900: DEVANAGARI SIGN INVERTED CANDRABINDU
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct IndicSyllabicCategory(pub(crate) u8);

impl IndicSyllabicCategory {
    /// Returns an ICU4C `UIndicSyllabicCategory` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UIndicSyllabicCategory` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl IndicSyllabicCategory {
    #[default]
    pub const Other: IndicSyllabicCategory = IndicSyllabicCategory(0);
 / "Other"    pub const Avagraha: IndicSyllabicCategory = IndicSyllabicCategory(1);
 / "Avagraha"    pub const Bindu: IndicSyllabicCategory = IndicSyllabicCategory(2);
 / "Bindu"    pub const BrahmiJoiningNumber: IndicSyllabicCategory = IndicSyllabicCategory(3);
 / "Brahmi_Joining_Number"    pub const CantillationMark: IndicSyllabicCategory = IndicSyllabicCategory(4);
 / "Cantillation_Mark"    pub const Consonant: IndicSyllabicCategory = IndicSyllabicCategory(5);
 / "Consonant"    pub const ConsonantDead: IndicSyllabicCategory = IndicSyllabicCategory(6);
 / "Consonant_Dead"    pub const ConsonantFinal: IndicSyllabicCategory = IndicSyllabicCategory(7);
 / "Consonant_Final"    pub const ConsonantHeadLetter: IndicSyllabicCategory = IndicSyllabicCategory(8);
 / "Consonant_Head_Letter"    pub const ConsonantInitialPostfixed: IndicSyllabicCategory = IndicSyllabicCategory(9);
 / "Consonant_Initial_Postfixed"    pub const ConsonantKiller: IndicSyllabicCategory = IndicSyllabicCategory(10); / "Consonant_Killer"
    pub const ConsonantMedial: IndicSyllabicCategory = IndicSyllabicCategory(11); / "Consonant_Medial"
    pub const ConsonantPlaceholder: IndicSyllabicCategory = IndicSyllabicCategory(12); / "Consonant_Placeholder"
    pub const ConsonantPrecedingRepha: IndicSyllabicCategory = IndicSyllabicCategory(13); / "Consonant_Preceding_Repha"
    pub const ConsonantPrefixed: IndicSyllabicCategory = IndicSyllabicCategory(14); / "Consonant_Prefixed"
    pub const ConsonantSubjoined: IndicSyllabicCategory = IndicSyllabicCategory(15); / "Consonant_Subjoined"
    pub const ConsonantSucceedingRepha: IndicSyllabicCategory = IndicSyllabicCategory(16); / "Consonant_Succeeding_Repha"
    pub const ConsonantWithStacker: IndicSyllabicCategory = IndicSyllabicCategory(17); / "Consonant_With_Stacker"
    pub const GeminationMark: IndicSyllabicCategory = IndicSyllabicCategory(18); / "Gemination_Mark"
    pub const InvisibleStacker: IndicSyllabicCategory = IndicSyllabicCategory(19); / "Invisible_Stacker"
    pub const Joiner: IndicSyllabicCategory = IndicSyllabicCategory(20); / "Joiner"
    pub const ModifyingLetter: IndicSyllabicCategory = IndicSyllabicCategory(21); / "Modifying_Letter"
    pub const NonJoiner: IndicSyllabicCategory = IndicSyllabicCategory(22); / "Non_Joiner"
    pub const Nukta: IndicSyllabicCategory = IndicSyllabicCategory(23); / "Nukta"
    pub const Number: IndicSyllabicCategory = IndicSyllabicCategory(24); / "Number"
    pub const NumberJoiner: IndicSyllabicCategory = IndicSyllabicCategory(25); / "Number_Joiner"
    pub const PureKiller: IndicSyllabicCategory = IndicSyllabicCategory(26); / "Pure_Killer"
    pub const RegisterShifter: IndicSyllabicCategory = IndicSyllabicCategory(27); / "Register_Shifter"
    pub const SyllableModifier: IndicSyllabicCategory = IndicSyllabicCategory(28); / "Syllable_Modifier"
    pub const ToneLetter: IndicSyllabicCategory = IndicSyllabicCategory(29); / "Tone_Letter"
    pub const ToneMark: IndicSyllabicCategory = IndicSyllabicCategory(30); / "Tone_Mark"
    pub const Virama: IndicSyllabicCategory = IndicSyllabicCategory(31); / "Virama"
    pub const Visarga: IndicSyllabicCategory = IndicSyllabicCategory(32); / "Visarga"
    pub const Vowel: IndicSyllabicCategory = IndicSyllabicCategory(33); / "Vowel"
    pub const VowelDependent: IndicSyllabicCategory = IndicSyllabicCategory(34); / "Vowel_Dependent"
    pub const VowelIndependent: IndicSyllabicCategory = IndicSyllabicCategory(35); / "Vowel_Independent"
    pub const ReorderingKiller: IndicSyllabicCategory = IndicSyllabicCategory(36); / "Reordering_Killer"
}
#[test]
fn indic_syllabic_category_consts();
}

make_enumerated_property! {
    name: "Indic_Syllabic_Category";
    short_name: "InSC";
    ident: IndicSyllabicCategory;
    data_marker: crate::provider::PropertyEnumIndicSyllabicCategoryV1;
    singleton: SINGLETON_PROPERTY_ENUM_INDIC_SYLLABIC_CATEGORY_V1;
    ule_ty: u8;
}

/// Enumerated property `Joining_Group`.
///
/// See Section 9.2, Arabic Joining Groups in The Unicode Standard for the summary of
/// each property value.
///
/// ```
/// use icu::properties::{props::JoiningGroup, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<JoiningGroup>::new().get('ع'),
///     JoiningGroup::Ain,
/// ); // U+0639: Arabic Letter Ain
/// assert_eq!(
///     CodePointMapData::<JoiningGroup>::new().get('ظ'),
///     JoiningGroup::Tah,
/// ); // U+0638: Arabic Letter Zah
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct JoiningGroup(pub(crate) u8);

impl JoiningGroup {
    /// Returns an ICU4C `UJoiningType` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UJoiningType` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl JoiningGroup {
    #[default]
    pub const NoJoiningGroup: JoiningGroup = JoiningGroup(0); / "No_Joining_Group"
    pub const Ain: JoiningGroup = JoiningGroup(1); / "Ain"
    pub const Alaph: JoiningGroup = JoiningGroup(2); / "Alaph"
    pub const Alef: JoiningGroup = JoiningGroup(3); / "Alef"
    pub const Beh: JoiningGroup = JoiningGroup(4); / "Beh"
    pub const Beth: JoiningGroup = JoiningGroup(5); / "Beth"
    pub const Dal: JoiningGroup = JoiningGroup(6); / "Dal"
    pub const DalathRish: JoiningGroup = JoiningGroup(7); / "Dalath_Rish"
    pub const E: JoiningGroup = JoiningGroup(8); / "E"
    pub const Feh: JoiningGroup = JoiningGroup(9); / "Feh"
    pub const FinalSemkath: JoiningGroup = JoiningGroup(10); / "Final_Semkath"
    pub const Gaf: JoiningGroup = JoiningGroup(11); / "Gaf"
    pub const Gamal: JoiningGroup = JoiningGroup(12); / "Gamal"
    pub const Hah: JoiningGroup = JoiningGroup(13); / "Hah"
    pub const TehMarbutaGoal: JoiningGroup = JoiningGroup(14); / "Teh_Marbuta_Goal"
    pub const He: JoiningGroup = JoiningGroup(15); / "He"
    pub const Heh: JoiningGroup = JoiningGroup(16); / "Heh"
    pub const HehGoal: JoiningGroup = JoiningGroup(17); / "Heh_Goal"
    pub const Heth: JoiningGroup = JoiningGroup(18); / "Heth"
    pub const Kaf: JoiningGroup = JoiningGroup(19); / "Kaf"
    pub const Kaph: JoiningGroup = JoiningGroup(20); / "Kaph"
    pub const KnottedHeh: JoiningGroup = JoiningGroup(21); / "Knotted_Heh"
    pub const Lam: JoiningGroup = JoiningGroup(22); / "Lam"
    pub const Lamadh: JoiningGroup = JoiningGroup(23); / "Lamadh"
    pub const Meem: JoiningGroup = JoiningGroup(24); / "Meem"
    pub const Mim: JoiningGroup = JoiningGroup(25); / "Mim"
    pub const Noon: JoiningGroup = JoiningGroup(26); / "Noon"
    pub const Nun: JoiningGroup = JoiningGroup(27); / "Nun"
    pub const Pe: JoiningGroup = JoiningGroup(28); / "Pe"
    pub const Qaf: JoiningGroup = JoiningGroup(29); / "Qaf"
    pub const Qaph: JoiningGroup = JoiningGroup(30); / "Qaph"
    pub const Reh: JoiningGroup = JoiningGroup(31); / "Reh"
    pub const ReversedPe: JoiningGroup = JoiningGroup(32); / "Reversed_Pe"
    pub const Sad: JoiningGroup = JoiningGroup(33); / "Sad"
    pub const Sadhe: JoiningGroup = JoiningGroup(34); / "Sadhe"
    pub const Seen: JoiningGroup = JoiningGroup(35); / "Seen"
    pub const Semkath: JoiningGroup = JoiningGroup(36); / "Semkath"
    pub const Shin: JoiningGroup = JoiningGroup(37); / "Shin"
    pub const SwashKaf: JoiningGroup = JoiningGroup(38); / "Swash_Kaf"
    pub const SyriacWaw: JoiningGroup = JoiningGroup(39); / "Syriac_Waw"
    pub const Tah: JoiningGroup = JoiningGroup(40); / "Tah"
    pub const Taw: JoiningGroup = JoiningGroup(41); / "Taw"
    pub const TehMarbuta: JoiningGroup = JoiningGroup(42); / "Teh_Marbuta"
    pub const Teth: JoiningGroup = JoiningGroup(43); / "Teth"
    pub const Waw: JoiningGroup = JoiningGroup(44); / "Waw"
    pub const Yeh: JoiningGroup = JoiningGroup(45); / "Yeh"
    pub const YehBarree: JoiningGroup = JoiningGroup(46); / "Yeh_Barree"
    pub const YehWithTail: JoiningGroup = JoiningGroup(47); / "Yeh_With_Tail"
    pub const Yudh: JoiningGroup = JoiningGroup(48); / "Yudh"
    pub const YudhHe: JoiningGroup = JoiningGroup(49); / "Yudh_He"
    pub const Zain: JoiningGroup = JoiningGroup(50); / "Zain"
    pub const Fe: JoiningGroup = JoiningGroup(51); / "Fe"
    pub const Khaph: JoiningGroup = JoiningGroup(52); / "Khaph"
    pub const Zhain: JoiningGroup = JoiningGroup(53); / "Zhain"
    pub const BurushaskiYehBarree: JoiningGroup = JoiningGroup(54); / "Burushaski_Yeh_Barree"
    pub const FarsiYeh: JoiningGroup = JoiningGroup(55); / "Farsi_Yeh"
    pub const Nya: JoiningGroup = JoiningGroup(56); / "Nya"
    pub const RohingyaYeh: JoiningGroup = JoiningGroup(57); / "Rohingya_Yeh"
    pub const ManichaeanAleph: JoiningGroup = JoiningGroup(58); / "Manichaean_Aleph"
    pub const ManichaeanAyin: JoiningGroup = JoiningGroup(59); / "Manichaean_Ayin"
    pub const ManichaeanBeth: JoiningGroup = JoiningGroup(60); / "Manichaean_Beth"
    pub const ManichaeanDaleth: JoiningGroup = JoiningGroup(61); / "Manichaean_Daleth"
    pub const ManichaeanDhamedh: JoiningGroup = JoiningGroup(62); / "Manichaean_Dhamedh"
    pub const ManichaeanFive: JoiningGroup = JoiningGroup(63); / "Manichaean_Five"
    pub const ManichaeanGimel: JoiningGroup = JoiningGroup(64); / "Manichaean_Gimel"
    pub const ManichaeanHeth: JoiningGroup = JoiningGroup(65); / "Manichaean_Heth"
    pub const ManichaeanHundred: JoiningGroup = JoiningGroup(66); / "Manichaean_Hundred"
    pub const ManichaeanKaph: JoiningGroup = JoiningGroup(67); / "Manichaean_Kaph"
    pub const ManichaeanLamedh: JoiningGroup = JoiningGroup(68); / "Manichaean_Lamedh"
    pub const ManichaeanMem: JoiningGroup = JoiningGroup(69); / "Manichaean_Mem"
    pub const ManichaeanNun: JoiningGroup = JoiningGroup(70); / "Manichaean_Nun"
    pub const ManichaeanOne: JoiningGroup = JoiningGroup(71); / "Manichaean_One"
    pub const ManichaeanPe: JoiningGroup = JoiningGroup(72); / "Manichaean_Pe"
    pub const ManichaeanQoph: JoiningGroup = JoiningGroup(73); / "Manichaean_Qoph"
    pub const ManichaeanResh: JoiningGroup = JoiningGroup(74); / "Manichaean_Resh"
    pub const ManichaeanSadhe: JoiningGroup = JoiningGroup(75); / "Manichaean_Sadhe"
    pub const ManichaeanSamekh: JoiningGroup = JoiningGroup(76); / "Manichaean_Samekh"
    pub const ManichaeanTaw: JoiningGroup = JoiningGroup(77); / "Manichaean_Taw"
    pub const ManichaeanTen: JoiningGroup = JoiningGroup(78); / "Manichaean_Ten"
    pub const ManichaeanTeth: JoiningGroup = JoiningGroup(79); / "Manichaean_Teth"
    pub const ManichaeanThamedh: JoiningGroup = JoiningGroup(80); / "Manichaean_Thamedh"
    pub const ManichaeanTwenty: JoiningGroup = JoiningGroup(81); / "Manichaean_Twenty"
    pub const ManichaeanWaw: JoiningGroup = JoiningGroup(82); / "Manichaean_Waw"
    pub const ManichaeanYodh: JoiningGroup = JoiningGroup(83); / "Manichaean_Yodh"
    pub const ManichaeanZayin: JoiningGroup = JoiningGroup(84); / "Manichaean_Zayin"
    pub const StraightWaw: JoiningGroup = JoiningGroup(85); / "Straight_Waw"
    pub const AfricanFeh: JoiningGroup = JoiningGroup(86); / "African_Feh"
    pub const AfricanNoon: JoiningGroup = JoiningGroup(87); / "African_Noon"
    pub const AfricanQaf: JoiningGroup = JoiningGroup(88); / "African_Qaf"
    pub const MalayalamBha: JoiningGroup = JoiningGroup(89); / "Malayalam_Bha"
    pub const MalayalamJa: JoiningGroup = JoiningGroup(90); / "Malayalam_Ja"
    pub const MalayalamLla: JoiningGroup = JoiningGroup(91); / "Malayalam_Lla"
    pub const MalayalamLlla: JoiningGroup = JoiningGroup(92); / "Malayalam_Llla"
    pub const MalayalamNga: JoiningGroup = JoiningGroup(93); / "Malayalam_Nga"
    pub const MalayalamNna: JoiningGroup = JoiningGroup(94); / "Malayalam_Nna"
    pub const MalayalamNnna: JoiningGroup = JoiningGroup(95); / "Malayalam_Nnna"
    pub const MalayalamNya: JoiningGroup = JoiningGroup(96); / "Malayalam_Nya"
    pub const MalayalamRa: JoiningGroup = JoiningGroup(97); / "Malayalam_Ra"
    pub const MalayalamSsa: JoiningGroup = JoiningGroup(98); / "Malayalam_Ssa"
    pub const MalayalamTta: JoiningGroup = JoiningGroup(99); / "Malayalam_Tta"
    pub const HanifiRohingyaKinnaYa: JoiningGroup = JoiningGroup(100); / "Hanifi_Rohingya_Kinna_Ya"
    pub const HanifiRohingyaPa: JoiningGroup = JoiningGroup(101); / "Hanifi_Rohingya_Pa"
    pub const ThinYeh: JoiningGroup = JoiningGroup(102); / "Thin_Yeh"
    pub const VerticalTail: JoiningGroup = JoiningGroup(103); / "Vertical_Tail"
    pub const KashmiriYeh: JoiningGroup = JoiningGroup(104); / "Kashmiri_Yeh"
    pub const ThinNoon: JoiningGroup = JoiningGroup(105); / "Thin_Noon"
}
#[test]
fn joining_group_consts();
}

make_enumerated_property! {
    name: "Joining_Group";
    short_name: "jg";
    ident: JoiningGroup;
    data_marker: crate::provider::PropertyEnumJoiningGroupV1;
    singleton: SINGLETON_PROPERTY_ENUM_JOINING_GROUP_V1;
    ule_ty: u8;
}

/// Enumerated property `Joining_Type`.
///
/// See Section 9.2, Arabic Cursive Joining in The Unicode Standard for the summary of
/// each property value.
///
/// # Example
///
/// ```
/// use icu::properties::{props::JoiningType, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<JoiningType>::new().get('ؠ'),
///     JoiningType::DualJoining
/// ); // U+0620: Arabic Letter Kashmiri Yeh
/// assert_eq!(
///     CodePointMapData::<JoiningType>::new().get('𐫍'),
///     JoiningType::LeftJoining
/// ); // U+10ACD: Manichaean Letter Heth
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct JoiningType(pub(crate) u8);

impl JoiningType {
    /// Returns an ICU4C `UJoiningType` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UJoiningType` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl JoiningType {
    #[default]
    pub const NonJoining: JoiningType = JoiningType(0); / "U"
    pub const JoinCausing: JoiningType = JoiningType(1); / "C"
    pub const DualJoining: JoiningType = JoiningType(2); / "D"
    pub const LeftJoining: JoiningType = JoiningType(3); / "L"
    pub const RightJoining: JoiningType = JoiningType(4); / "R"
    pub const Transparent: JoiningType = JoiningType(5); / "T"
}
#[test]
fn joining_type_consts();
}

make_enumerated_property! {
    name: "Joining_Type";
    short_name: "jt";
    ident: JoiningType;
    data_marker: crate::provider::PropertyEnumJoiningTypeV1;
    singleton: SINGLETON_PROPERTY_ENUM_JOINING_TYPE_V1;
    ule_ty: u8;
}

/// Property `Vertical_Orientation`
///
/// See UTR #50:
/// <https://www.unicode.org/reports/tr50/#vo>
///
/// # Example
///
/// ```
/// use icu::properties::{props::VerticalOrientation, CodePointMapData};
///
/// assert_eq!(
///     CodePointMapData::<VerticalOrientation>::new().get('a'),
///     VerticalOrientation::Rotated
/// );
/// assert_eq!(
///     CodePointMapData::<VerticalOrientation>::new().get('§'),
///     VerticalOrientation::Upright
/// );
/// assert_eq!(
///     CodePointMapData::<VerticalOrientation>::new().get32(0x2329),
///     VerticalOrientation::TransformedRotated
/// );
/// assert_eq!(
///     CodePointMapData::<VerticalOrientation>::new().get32(0x3001),
///     VerticalOrientation::TransformedUpright
/// );
/// ```
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
pub struct VerticalOrientation(pub(crate) u8);

impl VerticalOrientation {
    /// Returns an ICU4C `UVerticalOrientation` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn to_icu4c_value(self) -> u8 {
        self.0
    }
    /// Constructor from an ICU4C `UVerticalOrientation` value.
    #[deprecated(
        since = "2.3.0",
        note = "please comment on https://github.com/unicode-org/icu4x/issues/6067 if you need this"
    )]
    pub const fn from_icu4c_value(value: u8) -> Self {
        Self(value)
    }
}

create_const_array! {
#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl VerticalOrientation {
    #[default]
    pub const Rotated: VerticalOrientation = VerticalOrientation(0); / "R"
    pub const TransformedRotated: VerticalOrientation = VerticalOrientation(1); / "Tr"
    pub const TransformedUpright: VerticalOrientation = VerticalOrientation(2); / "Tu"
    pub const Upright: VerticalOrientation = VerticalOrientation(3); / "U"
}
#[test]
fn vertical_orientation_consts();
}

make_enumerated_property! {
    name: "Vertical_Orientation";
    short_name: "vo";
    ident: VerticalOrientation;
    data_marker: crate::provider::PropertyEnumVerticalOrientationV1;
    singleton: SINGLETON_PROPERTY_ENUM_VERTICAL_ORIENTATION_V1;
    ule_ty: u8;
}

pub use crate::code_point_set::BinaryProperty;

macro_rules! make_binary_property {
    (
        name: $name:literal;
        short_name: $short_name:literal;
        ident: $ident:ident;
        data_marker: $data_marker:ty;
        singleton: $singleton:ident;
            $(#[$doc:meta])+
    ) => {
        $(#[$doc])+
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct $ident;

        #[allow(deprecated)]
        impl crate::private::Sealed for $ident {}

        #[allow(deprecated)]
        impl BinaryProperty for $ident {
            type DataMarker = $data_marker;
            #[cfg(feature = "compiled_data")]
            const SINGLETON: &'static crate::provider::PropertyCodePointSet<'static> =
                &crate::provider::Baked::$singleton;
            const NAME: &'static [u8] = $name.as_bytes();
            const SHORT_NAME: &'static [u8] = $short_name.as_bytes();
        }
    };
}

make_binary_property! {
    name: "ASCII_Hex_Digit";
    short_name: "AHex";
    ident: AsciiHexDigit;
    data_marker: crate::provider::PropertyBinaryAsciiHexDigitV1;
    singleton: SINGLETON_PROPERTY_BINARY_ASCII_HEX_DIGIT_V1;
    /// ASCII characters commonly used for the representation of hexadecimal numbers.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::AsciiHexDigit;
    ///
    /// let ascii_hex_digit = CodePointSetData::new::<AsciiHexDigit>();
    ///
    /// assert!(ascii_hex_digit.contains('3'));
    /// assert!(!ascii_hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(ascii_hex_digit.contains('A'));
    /// assert!(!ascii_hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```
}

make_binary_property! {
    name: "alnum";
    short_name: "alnum";
    ident: Alnum;
    data_marker: crate::provider::PropertyBinaryAlnumV1;
    singleton: SINGLETON_PROPERTY_BINARY_ALNUM_V1;
    /// Characters with the `Alphabetic` or `Decimal_Number` property.
    ///
    /// This is defined for POSIX compatibility.
}

make_binary_property! {
    name: "Alphabetic";
    short_name: "Alpha";
    ident: Alphabetic;
    data_marker: crate::provider::PropertyBinaryAlphabeticV1;
    singleton: SINGLETON_PROPERTY_BINARY_ALPHABETIC_V1;
    /// Alphabetic characters.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Alphabetic;
    ///
    /// let alphabetic = CodePointSetData::new::<Alphabetic>();
    ///
    /// assert!(!alphabetic.contains('3'));
    /// assert!(!alphabetic.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(alphabetic.contains('A'));
    /// assert!(alphabetic.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```

}

make_binary_property! {
    name: "Bidi_Control";
    short_name: "Bidi_C";
    ident: BidiControl;
    data_marker: crate::provider::PropertyBinaryBidiControlV1;
    singleton: SINGLETON_PROPERTY_BINARY_BIDI_CONTROL_V1;
    /// Format control characters which have specific functions in the Unicode Bidirectional
    /// Algorithm.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::BidiControl;
    ///
    /// let bidi_control = CodePointSetData::new::<BidiControl>();
    ///
    /// assert!(bidi_control.contains('\u{200F}'));  // RIGHT-TO-LEFT MARK
    /// assert!(!bidi_control.contains('ش'));  // U+0634 ARABIC LETTER SHEEN
    /// ```

}

make_binary_property! {
    name: "Bidi_Mirrored";
    short_name: "Bidi_M";
    ident: BidiMirrored;
    data_marker: crate::provider::PropertyBinaryBidiMirroredV1;
    singleton: SINGLETON_PROPERTY_BINARY_BIDI_MIRRORED_V1;
    /// Characters that are mirrored in bidirectional text.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::BidiMirrored;
    ///
    /// let bidi_mirrored = CodePointSetData::new::<BidiMirrored>();
    ///
    /// assert!(bidi_mirrored.contains('['));
    /// assert!(bidi_mirrored.contains(']'));
    /// assert!(bidi_mirrored.contains('∑'));  // U+2211 N-ARY SUMMATION
    /// assert!(!bidi_mirrored.contains('ཉ'));  // U+0F49 TIBETAN LETTER NYA
    /// ```

}

make_binary_property! {
    name: "blank";
    short_name: "blank";
    ident: Blank;
    data_marker: crate::provider::PropertyBinaryBlankV1;
    singleton: SINGLETON_PROPERTY_BINARY_BLANK_V1;
    /// Horizontal whitespace characters

}

make_binary_property! {
    name: "Cased";
    short_name: "Cased";
    ident: Cased;
    data_marker: crate::provider::PropertyBinaryCasedV1;
    singleton: SINGLETON_PROPERTY_BINARY_CASED_V1;
    /// Uppercase, lowercase, and titlecase characters.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Cased;
    ///
    /// let cased = CodePointSetData::new::<Cased>();
    ///
    /// assert!(cased.contains('Ꙡ'));  // U+A660 CYRILLIC CAPITAL LETTER REVERSED TSE
    /// assert!(!cased.contains('ދ'));  // U+078B THAANA LETTER DHAALU
    /// ```

}

make_binary_property! {
    name: "Case_Ignorable";
    short_name: "CI";
    ident: CaseIgnorable;
    data_marker: crate::provider::PropertyBinaryCaseIgnorableV1;
    singleton: SINGLETON_PROPERTY_BINARY_CASE_IGNORABLE_V1;
    /// Characters which are ignored for casing purposes.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::CaseIgnorable;
    ///
    /// let case_ignorable = CodePointSetData::new::<CaseIgnorable>();
    ///
    /// assert!(case_ignorable.contains(':'));
    /// assert!(!case_ignorable.contains('λ'));  // U+03BB GREEK SMALL LETTER LAMBDA
    /// ```

}

make_binary_property! {
    name: "Full_Composition_Exclusion";
    short_name: "Comp_Ex";
    ident: FullCompositionExclusion;
    data_marker: crate::provider::PropertyBinaryFullCompositionExclusionV1;
    singleton: SINGLETON_PROPERTY_BINARY_FULL_COMPOSITION_EXCLUSION_V1;
    /// Characters that are excluded from composition.
    ///
    /// See <https://unicode.org/Public/UNIDATA/CompositionExclusions.txt>

}

make_binary_property! {
    name: "Changes_When_Casefolded";
    short_name: "CWCF";
    ident: ChangesWhenCasefolded;
    data_marker: crate::provider::PropertyBinaryChangesWhenCasefoldedV1;
    singleton: SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_CASEFOLDED_V1;
    /// Characters whose normalized forms are not stable under case folding.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::ChangesWhenCasefolded;
    ///
    /// let changes_when_casefolded = CodePointSetData::new::<ChangesWhenCasefolded>();
    ///
    /// assert!(changes_when_casefolded.contains('ß'));  // U+00DF LATIN SMALL LETTER SHARP S
    /// assert!(!changes_when_casefolded.contains('ᜉ'));  // U+1709 TAGALOG LETTER PA
    /// ```

}

make_binary_property! {
    name: "Changes_When_Casemapped";
    short_name: "CWCM";
    ident: ChangesWhenCasemapped;
    data_marker: crate::provider::PropertyBinaryChangesWhenCasemappedV1;
    singleton: SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_CASEMAPPED_V1;
    /// Characters which may change when they undergo case mapping.

}

make_binary_property! {
    name: "Changes_When_NFKC_Casefolded";
    short_name: "CWKCF";
    ident: ChangesWhenNfkcCasefolded;
    data_marker: crate::provider::PropertyBinaryChangesWhenNfkcCasefoldedV1;
    singleton: SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_NFKC_CASEFOLDED_V1;
    /// Characters which are not identical to their `NFKC_Casefold` mapping.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::ChangesWhenNfkcCasefolded;
    ///
    /// let changes_when_nfkc_casefolded = CodePointSetData::new::<ChangesWhenNfkcCasefolded>();
    ///
    /// assert!(changes_when_nfkc_casefolded.contains('🄵'));  // U+1F135 SQUARED LATIN CAPITAL LETTER F
    /// assert!(!changes_when_nfkc_casefolded.contains('f'));
    /// ```

}

make_binary_property! {
    name: "Changes_When_Lowercased";
    short_name: "CWL";
    ident: ChangesWhenLowercased;
    data_marker: crate::provider::PropertyBinaryChangesWhenLowercasedV1;
    singleton: SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_LOWERCASED_V1;
    /// Characters whose normalized forms are not stable under a `toLowercase` mapping.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::ChangesWhenLowercased;
    ///
    /// let changes_when_lowercased = CodePointSetData::new::<ChangesWhenLowercased>();
    ///
    /// assert!(changes_when_lowercased.contains('Ⴔ'));  // U+10B4 GEORGIAN CAPITAL LETTER PHAR
    /// assert!(!changes_when_lowercased.contains('ფ'));  // U+10E4 GEORGIAN LETTER PHAR
    /// ```

}

make_binary_property! {
    name: "Changes_When_Titlecased";
    short_name: "CWT";
    ident: ChangesWhenTitlecased;
    data_marker: crate::provider::PropertyBinaryChangesWhenTitlecasedV1;
    singleton: SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_TITLECASED_V1;
    /// Characters whose normalized forms are not stable under a `toTitlecase` mapping.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::ChangesWhenTitlecased;
    ///
    /// let changes_when_titlecased = CodePointSetData::new::<ChangesWhenTitlecased>();
    ///
    /// assert!(changes_when_titlecased.contains('æ'));  // U+00E6 LATIN SMALL LETTER AE
    /// assert!(!changes_when_titlecased.contains('Æ'));  // U+00E6 LATIN CAPITAL LETTER AE
    /// ```

}

make_binary_property! {
    name: "Changes_When_Uppercased";
    short_name: "CWU";
    ident: ChangesWhenUppercased;
    data_marker: crate::provider::PropertyBinaryChangesWhenUppercasedV1;
    singleton: SINGLETON_PROPERTY_BINARY_CHANGES_WHEN_UPPERCASED_V1;
    /// Characters whose normalized forms are not stable under a `toUppercase` mapping.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::ChangesWhenUppercased;
    ///
    /// let changes_when_uppercased = CodePointSetData::new::<ChangesWhenUppercased>();
    ///
    /// assert!(changes_when_uppercased.contains('ւ'));  // U+0582 ARMENIAN SMALL LETTER YIWN
    /// assert!(!changes_when_uppercased.contains('Ւ'));  // U+0552 ARMENIAN CAPITAL LETTER YIWN
    /// ```

}

make_binary_property! {
    name: "Dash";
    short_name: "Dash";
    ident: Dash;
    data_marker: crate::provider::PropertyBinaryDashV1;
    singleton: SINGLETON_PROPERTY_BINARY_DASH_V1;
    /// Punctuation characters explicitly called out as dashes in the Unicode Standard, plus
    /// their compatibility equivalents.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Dash;
    ///
    /// let dash = CodePointSetData::new::<Dash>();
    ///
    /// assert!(dash.contains('⸺'));  // U+2E3A TWO-EM DASH
    /// assert!(dash.contains('-'));  // U+002D
    /// assert!(!dash.contains('='));  // U+003D
    /// ```

}

make_binary_property! {
    name: "Deprecated";
    short_name: "Dep";
    ident: Deprecated;
    data_marker: crate::provider::PropertyBinaryDeprecatedV1;
    singleton: SINGLETON_PROPERTY_BINARY_DEPRECATED_V1;
    /// Deprecated characters.
    ///
    /// No characters will ever be removed from the standard, but the
    /// usage of deprecated characters is strongly discouraged.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Deprecated;
    ///
    /// let deprecated = CodePointSetData::new::<Deprecated>();
    ///
    /// assert!(deprecated.contains('ឣ'));  // U+17A3 KHMER INDEPENDENT VOWEL QAQ
    /// assert!(!deprecated.contains('A'));
    /// ```

}

make_binary_property! {
    name: "Default_Ignorable_Code_Point";
    short_name: "DI";
    ident: DefaultIgnorableCodePoint;
    data_marker: crate::provider::PropertyBinaryDefaultIgnorableCodePointV1;
    singleton: SINGLETON_PROPERTY_BINARY_DEFAULT_IGNORABLE_CODE_POINT_V1;
    /// For programmatic determination of default ignorable code points.
    ///
    /// New characters that
    /// should be ignored in rendering (unless explicitly supported) will be assigned in these
    /// ranges, permitting programs to correctly handle the default rendering of such
    /// characters when not otherwise supported.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::DefaultIgnorableCodePoint;
    ///
    /// let default_ignorable_code_point = CodePointSetData::new::<DefaultIgnorableCodePoint>();
    ///
    /// assert!(default_ignorable_code_point.contains('\u{180B}'));  // MONGOLIAN FREE VARIATION SELECTOR ONE
    /// assert!(!default_ignorable_code_point.contains('E'));
    /// ```

}

make_binary_property! {
    name: "Diacritic";
    short_name: "Dia";
    ident: Diacritic;
    data_marker: crate::provider::PropertyBinaryDiacriticV1;
    singleton: SINGLETON_PROPERTY_BINARY_DIACRITIC_V1;
    /// Characters that linguistically modify the meaning of another character to which they apply.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Diacritic;
    ///
    /// let diacritic = CodePointSetData::new::<Diacritic>();
    ///
    /// assert!(diacritic.contains('\u{05B3}'));  // HEBREW POINT HATAF QAMATS
    /// assert!(!diacritic.contains('א'));  // U+05D0 HEBREW LETTER ALEF
    /// ```

}

make_binary_property! {
    name: "Emoji_Modifier_Base";
    short_name: "EBase";
    ident: EmojiModifierBase;
    data_marker: crate::provider::PropertyBinaryEmojiModifierBaseV1;
    singleton: SINGLETON_PROPERTY_BINARY_EMOJI_MODIFIER_BASE_V1;
    /// Characters that can serve as a base for emoji modifiers.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::EmojiModifierBase;
    ///
    /// let emoji_modifier_base = CodePointSetData::new::<EmojiModifierBase>();
    ///
    /// assert!(emoji_modifier_base.contains('✊'));  // U+270A RAISED FIST
    /// assert!(!emoji_modifier_base.contains('⛰'));  // U+26F0 MOUNTAIN
    /// ```

}

make_binary_property! {
    name: "Emoji_Component";
    short_name: "EComp";
    ident: EmojiComponent;
    data_marker: crate::provider::PropertyBinaryEmojiComponentV1;
    singleton: SINGLETON_PROPERTY_BINARY_EMOJI_COMPONENT_V1;
    /// Characters used in emoji sequences that normally do not appear on emoji keyboards as
    /// separate choices, such as base characters for emoji keycaps.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::EmojiComponent;
    ///
    /// let emoji_component = CodePointSetData::new::<EmojiComponent>();
    ///
    /// assert!(emoji_component.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T
    /// assert!(emoji_component.contains('\u{20E3}'));  // COMBINING ENCLOSING KEYCAP
    /// assert!(emoji_component.contains('7'));
    /// assert!(!emoji_component.contains('T'));
    /// ```

}

make_binary_property! {
    name: "Emoji_Modifier";
    short_name: "EMod";
    ident: EmojiModifier;
    data_marker: crate::provider::PropertyBinaryEmojiModifierV1;
    singleton: SINGLETON_PROPERTY_BINARY_EMOJI_MODIFIER_V1;
    /// Characters that are emoji modifiers.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::EmojiModifier;
    ///
    /// let emoji_modifier = CodePointSetData::new::<EmojiModifier>();
    ///
    /// assert!(emoji_modifier.contains('\u{1F3FD}'));  // EMOJI MODIFIER FITZPATRICK TYPE-4
    /// assert!(!emoji_modifier.contains('\u{200C}'));  // ZERO WIDTH NON-JOINER
    /// ```

}

make_binary_property! {
    name: "Emoji";
    short_name: "Emoji";
    ident: Emoji;
    data_marker: crate::provider::PropertyBinaryEmojiV1;
    singleton: SINGLETON_PROPERTY_BINARY_EMOJI_V1;
    /// Characters that are emoji.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Emoji;
    ///
    /// let emoji = CodePointSetData::new::<Emoji>();
    ///
    /// assert!(emoji.contains('🔥'));  // U+1F525 FIRE
    /// assert!(!emoji.contains('V'));
    /// ```

}

make_binary_property! {
    name: "Emoji_Presentation";
    short_name: "EPres";
    ident: EmojiPresentation;
    data_marker: crate::provider::PropertyBinaryEmojiPresentationV1;
    singleton: SINGLETON_PROPERTY_BINARY_EMOJI_PRESENTATION_V1;
    /// Characters that have emoji presentation by default.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::EmojiPresentation;
    ///
    /// let emoji_presentation = CodePointSetData::new::<EmojiPresentation>();
    ///
    /// assert!(emoji_presentation.contains('🦬')); // U+1F9AC BISON
    /// assert!(!emoji_presentation.contains('♻'));  // U+267B BLACK UNIVERSAL RECYCLING SYMBOL
    /// ```

}

make_binary_property! {
    name: "Extender";
    short_name: "Ext";
    ident: Extender;
    data_marker: crate::provider::PropertyBinaryExtenderV1;
    singleton: SINGLETON_PROPERTY_BINARY_EXTENDER_V1;
    /// Characters whose principal function is to extend the value of a preceding alphabetic
    /// character or to extend the shape of adjacent characters.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Extender;
    ///
    /// let extender = CodePointSetData::new::<Extender>();
    ///
    /// assert!(extender.contains('ヾ'));  // U+30FE KATAKANA VOICED ITERATION MARK
    /// assert!(extender.contains('ー'));  // U+30FC KATAKANA-HIRAGANA PROLONGED SOUND MARK
    /// assert!(!extender.contains('・'));  // U+30FB KATAKANA MIDDLE DOT
    /// ```

}

make_binary_property! {
    name: "Extended_Pictographic";
    short_name: "ExtPict";
    ident: ExtendedPictographic;
    data_marker: crate::provider::PropertyBinaryExtendedPictographicV1;
    singleton: SINGLETON_PROPERTY_BINARY_EXTENDED_PICTOGRAPHIC_V1;
    /// Pictographic symbols, as well as reserved ranges in blocks largely associated with
    /// emoji characters
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::ExtendedPictographic;
    ///
    /// let extended_pictographic = CodePointSetData::new::<ExtendedPictographic>();
    ///
    /// assert!(extended_pictographic.contains('🥳')); // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
    /// assert!(!extended_pictographic.contains('🇪'));  // U+1F1EA REGIONAL INDICATOR SYMBOL LETTER E
    /// ```

}

make_binary_property! {
    name: "graph";
    short_name: "graph";
    ident: Graph;
    data_marker: crate::provider::PropertyBinaryGraphV1;
    singleton: SINGLETON_PROPERTY_BINARY_GRAPH_V1;
    /// Invisible characters.
    ///
    /// This is defined for POSIX compatibility.

}

make_binary_property! {
    name: "Grapheme_Base";
    short_name: "Gr_Base";
    ident: GraphemeBase;
    data_marker: crate::provider::PropertyBinaryGraphemeBaseV1;
    singleton: SINGLETON_PROPERTY_BINARY_GRAPHEME_BASE_V1;
    /// Property used together with the definition of Standard Korean Syllable Block to define
    /// "Grapheme base".
    ///
    /// See D58 in Chapter 3, Conformance in the Unicode Standard.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::GraphemeBase;
    ///
    /// let grapheme_base = CodePointSetData::new::<GraphemeBase>();
    ///
    /// assert!(grapheme_base.contains('ക'));  // U+0D15 MALAYALAM LETTER KA
    /// assert!(grapheme_base.contains('\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I
    /// assert!(!grapheme_base.contains('\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA
    /// ```

}

make_binary_property! {
    name: "Grapheme_Extend";
    short_name: "Gr_Ext";
    ident: GraphemeExtend;
    data_marker: crate::provider::PropertyBinaryGraphemeExtendV1;
    singleton: SINGLETON_PROPERTY_BINARY_GRAPHEME_EXTEND_V1;
    /// Property used to define "Grapheme extender".
    ///
    /// See D59 in Chapter 3, Conformance in the
    /// Unicode Standard.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::GraphemeExtend;
    ///
    /// let grapheme_extend = CodePointSetData::new::<GraphemeExtend>();
    ///
    /// assert!(!grapheme_extend.contains('ക'));  // U+0D15 MALAYALAM LETTER KA
    /// assert!(!grapheme_extend.contains('\u{0D3F}'));  // U+0D3F MALAYALAM VOWEL SIGN I
    /// assert!(grapheme_extend.contains('\u{0D3E}'));  // U+0D3E MALAYALAM VOWEL SIGN AA
    /// ```

}

make_binary_property! {
    name: "Grapheme_Link";
    short_name: "Gr_Link";
    ident: GraphemeLink;
    data_marker: crate::provider::PropertyBinaryGraphemeLinkV1;
    singleton: SINGLETON_PROPERTY_BINARY_GRAPHEME_LINK_V1;
    /// Deprecated property.
    ///
    /// Formerly proposed for programmatic determination of grapheme
    /// cluster boundaries.
}

make_binary_property! {
    name: "Hex_Digit";
    short_name: "Hex";
    ident: HexDigit;
    data_marker: crate::provider::PropertyBinaryHexDigitV1;
    singleton: SINGLETON_PROPERTY_BINARY_HEX_DIGIT_V1;
    /// Characters commonly used for the representation of hexadecimal numbers, plus their
    /// compatibility equivalents.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::HexDigit;
    ///
    /// let hex_digit = CodePointSetData::new::<HexDigit>();
    ///
    /// assert!(hex_digit.contains('0'));
    /// assert!(!hex_digit.contains('੩'));  // U+0A69 GURMUKHI DIGIT THREE
    /// assert!(hex_digit.contains('f'));
    /// assert!(hex_digit.contains('ｆ'));  // U+FF46 FULLWIDTH LATIN SMALL LETTER F
    /// assert!(hex_digit.contains('Ｆ'));  // U+FF26 FULLWIDTH LATIN CAPITAL LETTER F
    /// assert!(!hex_digit.contains('Ä'));  // U+00C4 LATIN CAPITAL LETTER A WITH DIAERESIS
    /// ```
}

make_binary_property! {
    name: "Hyphen";
    short_name: "Hyphen";
    ident: Hyphen;
    data_marker: crate::provider::PropertyBinaryHyphenV1;
    singleton: SINGLETON_PROPERTY_BINARY_HYPHEN_V1;
    /// Deprecated property.
    ///
    /// Dashes which are used to mark connections between pieces of
    /// words, plus the Katakana middle dot.
}

make_binary_property! {
    name: "ID_Compat_Math_Continue";
    short_name: "ID_Compat_Math_Continue";
    ident: IdCompatMathContinue;
    data_marker: crate::provider::PropertyBinaryIdCompatMathContinueV1;
    singleton: SINGLETON_PROPERTY_BINARY_ID_COMPAT_MATH_CONTINUE_V1;
    /// `ID_Compat_Math_Continue` Property
}

make_binary_property! {
    name: "ID_Compat_Math_Start";
    short_name: "ID_Compat_Math_Start";
    ident: IdCompatMathStart;
    data_marker: crate::provider::PropertyBinaryIdCompatMathStartV1;
    singleton: SINGLETON_PROPERTY_BINARY_ID_COMPAT_MATH_START_V1;
    /// `ID_Compat_Math_Start` Property
}

make_binary_property! {
    name: "ID_Continue";
    short_name: "IDC";
    ident: IdContinue;
    data_marker: crate::provider::PropertyBinaryIdContinueV1;
    singleton: SINGLETON_PROPERTY_BINARY_ID_CONTINUE_V1;
    /// Characters that can come after the first character in an identifier.
    ///
    /// If using NFKC to
    /// fold differences between characters, use [`XidContinue`] instead.  See
    /// [`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
    /// more details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::IdContinue;
    ///
    /// let id_continue = CodePointSetData::new::<IdContinue>();
    ///
    /// assert!(id_continue.contains('x'));
    /// assert!(id_continue.contains('1'));
    /// assert!(id_continue.contains('_'));
    /// assert!(id_continue.contains('ߝ'));  // U+07DD NKO LETTER FA
    /// assert!(!id_continue.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
    /// assert!(id_continue.contains('\u{FC5E}'));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
    /// ```
}

make_binary_property! {
    name: "Ideographic";
    short_name: "Ideo";
    ident: Ideographic;
    data_marker: crate::provider::PropertyBinaryIdeographicV1;
    singleton: SINGLETON_PROPERTY_BINARY_IDEOGRAPHIC_V1;
    /// Characters considered to be CJKV (Chinese, Japanese, Korean, and Vietnamese)
    /// ideographs, or related siniform ideographs
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Ideographic;
    ///
    /// let ideographic = CodePointSetData::new::<Ideographic>();
    ///
    /// assert!(ideographic.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD
    /// assert!(!ideographic.contains('밥'));  // U+BC25 HANGUL SYLLABLE BAB
    /// ```
}

make_binary_property! {
    name: "ID_Start";
    short_name: "IDS";
    ident: IdStart;
    data_marker: crate::provider::PropertyBinaryIdStartV1;
    singleton: SINGLETON_PROPERTY_BINARY_ID_START_V1;
    /// Characters that can begin an identifier.
    ///
    /// If using NFKC to fold differences between
    /// characters, use [`XidStart`] instead.  See [`Unicode Standard Annex
    /// #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::IdStart;
    ///
    /// let id_start = CodePointSetData::new::<IdStart>();
    ///
    /// assert!(id_start.contains('x'));
    /// assert!(!id_start.contains('1'));
    /// assert!(!id_start.contains('_'));
    /// assert!(id_start.contains('ߝ'));  // U+07DD NKO LETTER FA
    /// assert!(!id_start.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
    /// assert!(id_start.contains('\u{FC5E}'));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
    /// ```
}

make_binary_property! {
    name: "IDS_Binary_Operator";
    short_name: "IDSB";
    ident: IdsBinaryOperator;
    data_marker: crate::provider::PropertyBinaryIdsBinaryOperatorV1;
    singleton: SINGLETON_PROPERTY_BINARY_IDS_BINARY_OPERATOR_V1;
    /// Characters used in Ideographic Description Sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::IdsBinaryOperator;
    ///
    /// let ids_binary_operator = CodePointSetData::new::<IdsBinaryOperator>();
    ///
    /// assert!(ids_binary_operator.contains('\u{2FF5}'));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE
    /// assert!(!ids_binary_operator.contains('\u{3006}'));  // IDEOGRAPHIC CLOSING MARK
    /// ```
}

make_binary_property! {
    name: "IDS_Trinary_Operator";
    short_name: "IDST";
    ident: IdsTrinaryOperator;
    data_marker: crate::provider::PropertyBinaryIdsTrinaryOperatorV1;
    singleton: SINGLETON_PROPERTY_BINARY_IDS_TRINARY_OPERATOR_V1;
    /// Characters used in Ideographic Description Sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::IdsTrinaryOperator;
    ///
    /// let ids_trinary_operator = CodePointSetData::new::<IdsTrinaryOperator>();
    ///
    /// assert!(ids_trinary_operator.contains('\u{2FF2}'));  // IDEOGRAPHIC DESCRIPTION CHARACTER LEFT TO MIDDLE AND RIGHT
    /// assert!(ids_trinary_operator.contains('\u{2FF3}'));  // IDEOGRAPHIC DESCRIPTION CHARACTER ABOVE TO MIDDLE AND BELOW
    /// assert!(!ids_trinary_operator.contains('\u{2FF4}'));
    /// assert!(!ids_trinary_operator.contains('\u{2FF5}'));  // IDEOGRAPHIC DESCRIPTION CHARACTER SURROUND FROM ABOVE
    /// assert!(!ids_trinary_operator.contains('\u{3006}'));  // IDEOGRAPHIC CLOSING MARK
    /// ```
}

make_binary_property! {
    name: "IDS_Unary_Operator";
    short_name: "IDSU";
    ident: IdsUnaryOperator;
    data_marker: crate::provider::PropertyBinaryIdsUnaryOperatorV1;
    singleton: SINGLETON_PROPERTY_BINARY_IDS_UNARY_OPERATOR_V1;
    /// `IDS_Unary_Operator` Property
}

make_binary_property! {
    name: "Join_Control";
    short_name: "Join_C";
    ident: JoinControl;
    data_marker: crate::provider::PropertyBinaryJoinControlV1;
    singleton: SINGLETON_PROPERTY_BINARY_JOIN_CONTROL_V1;
    /// Format control characters which have specific functions for control of cursive joining
    /// and ligation.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::JoinControl;
    ///
    /// let join_control = CodePointSetData::new::<JoinControl>();
    ///
    /// assert!(join_control.contains('\u{200C}'));  // ZERO WIDTH NON-JOINER
    /// assert!(join_control.contains('\u{200D}'));  // ZERO WIDTH JOINER
    /// assert!(!join_control.contains('\u{200E}'));
    /// ```
}

make_binary_property! {
    name: "Logical_Order_Exception";
    short_name: "LOE";
    ident: LogicalOrderException;
    data_marker: crate::provider::PropertyBinaryLogicalOrderExceptionV1;
    singleton: SINGLETON_PROPERTY_BINARY_LOGICAL_ORDER_EXCEPTION_V1;
    /// A small number of spacing vowel letters occurring in certain Southeast Asian scripts such as Thai and Lao.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::LogicalOrderException;
    ///
    /// let logical_order_exception = CodePointSetData::new::<LogicalOrderException>();
    ///
    /// assert!(logical_order_exception.contains('ແ'));  // U+0EC1 LAO VOWEL SIGN EI
    /// assert!(!logical_order_exception.contains('ະ'));  // U+0EB0 LAO VOWEL SIGN A
    /// ```
}

make_binary_property! {
    name: "Lowercase";
    short_name: "Lower";
    ident: Lowercase;
    data_marker: crate::provider::PropertyBinaryLowercaseV1;
    singleton: SINGLETON_PROPERTY_BINARY_LOWERCASE_V1;
    /// Lowercase characters.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Lowercase;
    ///
    /// let lowercase = CodePointSetData::new::<Lowercase>();
    ///
    /// assert!(lowercase.contains('a'));
    /// assert!(!lowercase.contains('A'));
    /// ```
}

make_binary_property! {
    name: "Math";
    short_name: "Math";
    ident: Math;
    data_marker: crate::provider::PropertyBinaryMathV1;
    singleton: SINGLETON_PROPERTY_BINARY_MATH_V1;
    /// Characters used in mathematical notation.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Math;
    ///
    /// let math = CodePointSetData::new::<Math>();
    ///
    /// assert!(math.contains('='));
    /// assert!(math.contains('+'));
    /// assert!(!math.contains('-'));
    /// assert!(math.contains('−'));  // U+2212 MINUS SIGN
    /// assert!(!math.contains('/'));
    /// assert!(math.contains('∕'));  // U+2215 DIVISION SLASH
    /// ```
}

make_binary_property! {
    name: "Modifier_Combining_Mark";
    short_name: "MCM";
    ident: ModifierCombiningMark;
    data_marker: crate::provider::PropertyBinaryModifierCombiningMarkV1;
    singleton: SINGLETON_PROPERTY_BINARY_MODIFIER_COMBINING_MARK_V1;
    /// `Modifier_Combining_Mark` Property
}

make_binary_property! {
    name: "Noncharacter_Code_Point";
    short_name: "NChar";
    ident: NoncharacterCodePoint;
    data_marker: crate::provider::PropertyBinaryNoncharacterCodePointV1;
    singleton: SINGLETON_PROPERTY_BINARY_NONCHARACTER_CODE_POINT_V1;
    /// Code points permanently reserved for internal use.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::NoncharacterCodePoint;
    ///
    /// let noncharacter_code_point = CodePointSetData::new::<NoncharacterCodePoint>();
    ///
    /// assert!(noncharacter_code_point.contains('\u{FDD0}'));
    /// assert!(noncharacter_code_point.contains('\u{FFFF}'));
    /// assert!(!noncharacter_code_point.contains('\u{10000}'));
    /// ```
}

make_binary_property! {
    name: "NFC_Inert";
    short_name: "nfcinert";
    ident: NfcInert;
    data_marker: crate::provider::PropertyBinaryNfcInertV1;
    singleton: SINGLETON_PROPERTY_BINARY_NFC_INERT_V1;
    /// Characters that are inert under NFC, i.e., they do not interact with adjacent characters.
    #[deprecated(since = "2.3.0", note = "not a UCD property")]
}

make_binary_property! {
    name: "NFD_Inert";
    short_name: "nfdinert";
    ident: NfdInert;
    data_marker: crate::provider::PropertyBinaryNfdInertV1;
    singleton: SINGLETON_PROPERTY_BINARY_NFD_INERT_V1;
    /// Characters that are inert under NFD, i.e., they do not interact with adjacent characters.
    #[deprecated(since = "2.3.0", note = "not a UCD property")]
}

make_binary_property! {
    name: "NFKC_Inert";
    short_name: "nfkcinert";
    ident: NfkcInert;
    data_marker: crate::provider::PropertyBinaryNfkcInertV1;
    singleton: SINGLETON_PROPERTY_BINARY_NFKC_INERT_V1;
    /// Characters that are inert under NFKC, i.e., they do not interact with adjacent characters.
    #[deprecated(since = "2.3.0", note = "not a UCD property")]
}

make_binary_property! {
    name: "NFKD_Inert";
    short_name: "nfkdinert";
    ident: NfkdInert;
    data_marker: crate::provider::PropertyBinaryNfkdInertV1;
    singleton: SINGLETON_PROPERTY_BINARY_NFKD_INERT_V1;
    /// Characters that are inert under NFKD, i.e., they do not interact with adjacent characters.
    #[deprecated(since = "2.3.0", note = "not a UCD property")]
}

make_binary_property! {
    name: "Pattern_Syntax";
    short_name: "Pat_Syn";
    ident: PatternSyntax;
    data_marker: crate::provider::PropertyBinaryPatternSyntaxV1;
    singleton: SINGLETON_PROPERTY_BINARY_PATTERN_SYNTAX_V1;
    /// Characters used as syntax in patterns (such as regular expressions).
    ///
    /// See [`Unicode
    /// Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
    /// details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::PatternSyntax;
    ///
    /// let pattern_syntax = CodePointSetData::new::<PatternSyntax>();
    ///
    /// assert!(pattern_syntax.contains('{'));
    /// assert!(pattern_syntax.contains('⇒'));  // U+21D2 RIGHTWARDS DOUBLE ARROW
    /// assert!(!pattern_syntax.contains('0'));
    /// ```
}

make_binary_property! {
    name: "Pattern_White_Space";
    short_name: "Pat_WS";
    ident: PatternWhiteSpace;
    data_marker: crate::provider::PropertyBinaryPatternWhiteSpaceV1;
    singleton: SINGLETON_PROPERTY_BINARY_PATTERN_WHITE_SPACE_V1;
    /// Characters used as whitespace in patterns (such as regular expressions).
    ///
    /// See
    /// [`Unicode Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for
    /// more details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::PatternWhiteSpace;
    ///
    /// let pattern_white_space = CodePointSetData::new::<PatternWhiteSpace>();
    ///
    /// assert!(pattern_white_space.contains(' '));
    /// assert!(pattern_white_space.contains('\u{2029}'));  // PARAGRAPH SEPARATOR
    /// assert!(pattern_white_space.contains('\u{000A}'));  // NEW LINE
    /// assert!(!pattern_white_space.contains('\u{00A0}'));  // NO-BREAK SPACE
    /// ```
}

make_binary_property! {
    name: "Prepended_Concatenation_Mark";
    short_name: "PCM";
    ident: PrependedConcatenationMark;
    data_marker: crate::provider::PropertyBinaryPrependedConcatenationMarkV1;
    singleton: SINGLETON_PROPERTY_BINARY_PREPENDED_CONCATENATION_MARK_V1;
    /// A small class of visible format controls, which precede and then span a sequence of
    /// other characters, usually digits.
}

make_binary_property! {
    name: "print";
    short_name: "print";
    ident: Print;
    data_marker: crate::provider::PropertyBinaryPrintV1;
    singleton: SINGLETON_PROPERTY_BINARY_PRINT_V1;
    /// Printable characters (visible characters and whitespace).
    ///
    /// This is defined for POSIX compatibility.
}

make_binary_property! {
    name: "Quotation_Mark";
    short_name: "QMark";
    ident: QuotationMark;
    data_marker: crate::provider::PropertyBinaryQuotationMarkV1;
    singleton: SINGLETON_PROPERTY_BINARY_QUOTATION_MARK_V1;
    /// Punctuation characters that function as quotation marks.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::QuotationMark;
    ///
    /// let quotation_mark = CodePointSetData::new::<QuotationMark>();
    ///
    /// assert!(quotation_mark.contains('\''));
    /// assert!(quotation_mark.contains('„'));  // U+201E DOUBLE LOW-9 QUOTATION MARK
    /// assert!(!quotation_mark.contains('<'));
    /// ```
}

make_binary_property! {
    name: "Radical";
    short_name: "Radical";
    ident: Radical;
    data_marker: crate::provider::PropertyBinaryRadicalV1;
    singleton: SINGLETON_PROPERTY_BINARY_RADICAL_V1;
    /// Characters used in the definition of Ideographic Description Sequences.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Radical;
    ///
    /// let radical = CodePointSetData::new::<Radical>();
    ///
    /// assert!(radical.contains('⺆'));  // U+2E86 CJK RADICAL BOX
    /// assert!(!radical.contains('丹'));  // U+F95E CJK COMPATIBILITY IDEOGRAPH-F95E
    /// ```
}

make_binary_property! {
    name: "Regional_Indicator";
    short_name: "RI";
    ident: RegionalIndicator;
    data_marker: crate::provider::PropertyBinaryRegionalIndicatorV1;
    singleton: SINGLETON_PROPERTY_BINARY_REGIONAL_INDICATOR_V1;
    /// Regional indicator characters, `U+1F1E6..U+1F1FF`.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::RegionalIndicator;
    ///
    /// let regional_indicator = CodePointSetData::new::<RegionalIndicator>();
    ///
    /// assert!(regional_indicator.contains('🇹'));  // U+1F1F9 REGIONAL INDICATOR SYMBOL LETTER T
    /// assert!(!regional_indicator.contains('Ⓣ'));  // U+24C9 CIRCLED LATIN CAPITAL LETTER T
    /// assert!(!regional_indicator.contains('T'));
    /// ```
}

make_binary_property! {
    name: "Soft_Dotted";
    short_name: "SD";
    ident: SoftDotted;
    data_marker: crate::provider::PropertyBinarySoftDottedV1;
    singleton: SINGLETON_PROPERTY_BINARY_SOFT_DOTTED_V1;
    /// Characters with a "soft dot", like i or j.
    ///
    /// An accent placed on these characters causes
    /// the dot to disappear.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::SoftDotted;
    ///
    /// let soft_dotted = CodePointSetData::new::<SoftDotted>();
    ///
    /// assert!(soft_dotted.contains('і'));  //U+0456 CYRILLIC SMALL LETTER BYELORUSSIAN-UKRAINIAN I
    /// assert!(!soft_dotted.contains('ı'));  // U+0131 LATIN SMALL LETTER DOTLESS I
    /// ```
}

make_binary_property! {
    name: "Segment_Starter";
    short_name: "segstart";
    ident: SegmentStarter;
    data_marker: crate::provider::PropertyBinarySegmentStarterV1;
    singleton: SINGLETON_PROPERTY_BINARY_SEGMENT_STARTER_V1;
    /// Characters that are starters in terms of Unicode normalization and combining character
    /// sequences.
    #[deprecated(since = "2.3.0", note = "not a UCD property")]
}

make_binary_property! {
    name: "Case_Sensitive";
    short_name: "Sensitive";
    ident: CaseSensitive;
    data_marker: crate::provider::PropertyBinaryCaseSensitiveV1;
    singleton: SINGLETON_PROPERTY_BINARY_CASE_SENSITIVE_V1;
    /// Characters that are either the source of a case mapping or in the target of a case
    /// mapping.
    #[deprecated(since = "2.3.0", note = "not a UCD property")]
}

make_binary_property! {
    name: "Sentence_Terminal";
    short_name: "STerm";
    ident: SentenceTerminal;
    data_marker: crate::provider::PropertyBinarySentenceTerminalV1;
    singleton: SINGLETON_PROPERTY_BINARY_SENTENCE_TERMINAL_V1;
    /// Punctuation characters that generally mark the end of sentences.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::SentenceTerminal;
    ///
    /// let sentence_terminal = CodePointSetData::new::<SentenceTerminal>();
    ///
    /// assert!(sentence_terminal.contains('.'));
    /// assert!(sentence_terminal.contains('?'));
    /// assert!(sentence_terminal.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN
    /// assert!(!sentence_terminal.contains(','));
    /// assert!(!sentence_terminal.contains('¿'));  // U+00BF INVERTED QUESTION MARK
    /// ```
}

make_binary_property! {
    name: "Terminal_Punctuation";
    short_name: "Term";
    ident: TerminalPunctuation;
    data_marker: crate::provider::PropertyBinaryTerminalPunctuationV1;
    singleton: SINGLETON_PROPERTY_BINARY_TERMINAL_PUNCTUATION_V1;
    /// Punctuation characters that generally mark the end of textual units.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::TerminalPunctuation;
    ///
    /// let terminal_punctuation = CodePointSetData::new::<TerminalPunctuation>();
    ///
    /// assert!(terminal_punctuation.contains('.'));
    /// assert!(terminal_punctuation.contains('?'));
    /// assert!(terminal_punctuation.contains('᪨'));  // U+1AA8 TAI THAM SIGN KAAN
    /// assert!(terminal_punctuation.contains(','));
    /// assert!(!terminal_punctuation.contains('¿'));  // U+00BF INVERTED QUESTION MARK
    /// ```
}

make_binary_property! {
    name: "Unified_Ideograph";
    short_name: "UIdeo";
    ident: UnifiedIdeograph;
    data_marker: crate::provider::PropertyBinaryUnifiedIdeographV1;
    singleton: SINGLETON_PROPERTY_BINARY_UNIFIED_IDEOGRAPH_V1;
    /// A property which specifies the exact set of Unified CJK Ideographs in the standard.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::UnifiedIdeograph;
    ///
    /// let unified_ideograph = CodePointSetData::new::<UnifiedIdeograph>();
    ///
    /// assert!(unified_ideograph.contains('川'));  // U+5DDD CJK UNIFIED IDEOGRAPH-5DDD
    /// assert!(unified_ideograph.contains('木'));  // U+6728 CJK UNIFIED IDEOGRAPH-6728
    /// assert!(!unified_ideograph.contains('𛅸'));  // U+1B178 NUSHU CHARACTER-1B178
    /// ```
}

make_binary_property! {
    name: "Uppercase";
    short_name: "Upper";
    ident: Uppercase;
    data_marker: crate::provider::PropertyBinaryUppercaseV1;
    singleton: SINGLETON_PROPERTY_BINARY_UPPERCASE_V1;
    /// Uppercase characters.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::Uppercase;
    ///
    /// let uppercase = CodePointSetData::new::<Uppercase>();
    ///
    /// assert!(uppercase.contains('U'));
    /// assert!(!uppercase.contains('u'));
    /// ```
}

make_binary_property! {
    name: "Variation_Selector";
    short_name: "VS";
    ident: VariationSelector;
    data_marker: crate::provider::PropertyBinaryVariationSelectorV1;
    singleton: SINGLETON_PROPERTY_BINARY_VARIATION_SELECTOR_V1;
    /// Characters that are Variation Selectors.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::VariationSelector;
    ///
    /// let variation_selector = CodePointSetData::new::<VariationSelector>();
    ///
    /// assert!(variation_selector.contains('\u{180D}'));  // MONGOLIAN FREE VARIATION SELECTOR THREE
    /// assert!(!variation_selector.contains('\u{303E}'));  // IDEOGRAPHIC VARIATION INDICATOR
    /// assert!(variation_selector.contains('\u{FE0F}'));  // VARIATION SELECTOR-16
    /// assert!(!variation_selector.contains('\u{FE10}'));  // PRESENTATION FORM FOR VERTICAL COMMA
    /// assert!(variation_selector.contains('\u{E01EF}'));  // VARIATION SELECTOR-256
    /// ```
}

make_binary_property! {
    name: "White_Space";
    short_name: "WSpace";
    ident: WhiteSpace;
    data_marker: crate::provider::PropertyBinaryWhiteSpaceV1;
    singleton: SINGLETON_PROPERTY_BINARY_WHITE_SPACE_V1;
    /// Spaces, separator characters and other control characters which should be treated by
    /// programming languages as "white space" for the purpose of parsing elements.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::WhiteSpace;
    ///
    /// let white_space = CodePointSetData::new::<WhiteSpace>();
    ///
    /// assert!(white_space.contains(' '));
    /// assert!(white_space.contains('\u{000A}'));  // NEW LINE
    /// assert!(white_space.contains('\u{00A0}'));  // NO-BREAK SPACE
    /// assert!(!white_space.contains('\u{200B}'));  // ZERO WIDTH SPACE
    /// ```
}

make_binary_property! {
    name: "xdigit";
    short_name: "xdigit";
    ident: Xdigit;
    data_marker: crate::provider::PropertyBinaryXdigitV1;
    singleton: SINGLETON_PROPERTY_BINARY_XDIGIT_V1;
    /// Hexadecimal digits
    ///
    /// This is defined for POSIX compatibility.
}

make_binary_property! {
    name: "XID_Continue";
    short_name: "XIDC";
    ident: XidContinue;
    data_marker: crate::provider::PropertyBinaryXidContinueV1;
    singleton: SINGLETON_PROPERTY_BINARY_XID_CONTINUE_V1;
    /// Characters that can come after the first character in an identifier.
    ///
    /// See [`Unicode Standard Annex
    /// #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::XidContinue;
    ///
    /// let xid_continue = CodePointSetData::new::<XidContinue>();
    ///
    /// assert!(xid_continue.contains('x'));
    /// assert!(xid_continue.contains('1'));
    /// assert!(xid_continue.contains('_'));
    /// assert!(xid_continue.contains('ߝ'));  // U+07DD NKO LETTER FA
    /// assert!(!xid_continue.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
    /// assert!(!xid_continue.contains('\u{FC5E}'));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
    /// ```
}

make_binary_property! {
    name: "XID_Start";
    short_name: "XIDS";
    ident: XidStart;
    data_marker: crate::provider::PropertyBinaryXidStartV1;
    singleton: SINGLETON_PROPERTY_BINARY_XID_START_V1;
    /// Characters that can begin an identifier.
    ///
    /// See [`Unicode
    /// Standard Annex #31`](https://www.unicode.org/reports/tr31/tr31-35.html) for more
    /// details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::CodePointSetData;
    /// use icu::properties::props::XidStart;
    ///
    /// let xid_start = CodePointSetData::new::<XidStart>();
    ///
    /// assert!(xid_start.contains('x'));
    /// assert!(!xid_start.contains('1'));
    /// assert!(!xid_start.contains('_'));
    /// assert!(xid_start.contains('ߝ'));  // U+07DD NKO LETTER FA
    /// assert!(!xid_start.contains('ⓧ'));  // U+24E7 CIRCLED LATIN SMALL LETTER X
    /// assert!(!xid_start.contains('\u{FC5E}'));  // ARABIC LIGATURE SHADDA WITH DAMMATAN ISOLATED FORM
    /// ```
}

pub use crate::emoji::EmojiSet;

macro_rules! make_emoji_set {
    (
        name: $name:literal;
        short_name: $short_name:literal;
        ident: $ident:ident;
        data_marker: $data_marker:ty;
        singleton: $singleton:ident;
        $(#[$doc:meta])+
    ) => {
        $(#[$doc])+
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct $ident;

        impl crate::private::Sealed for $ident {}

        impl EmojiSet for $ident {
            type DataMarker = $data_marker;
            #[cfg(feature = "compiled_data")]
            const SINGLETON: &'static crate::provider::PropertyUnicodeSet<'static> =
                &crate::provider::Baked::$singleton;
            const NAME: &'static [u8] = $name.as_bytes();
            const SHORT_NAME: &'static [u8] = $short_name.as_bytes();
        }
    }
}

make_emoji_set! {
    name: "Basic_Emoji";
    short_name: "Basic_Emoji";
    ident: BasicEmoji;
    data_marker: crate::provider::PropertyBinaryBasicEmojiV1;
    singleton: SINGLETON_PROPERTY_BINARY_BASIC_EMOJI_V1;
    /// Characters and character sequences intended for general-purpose, independent, direct input.
    ///
    /// See [`Unicode Technical Standard #51`](https://unicode.org/reports/tr51/) for more
    /// details.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::EmojiSetData;
    /// use icu::properties::props::BasicEmoji;
    ///
    /// let basic_emoji = EmojiSetData::new::<BasicEmoji>();
    ///
    /// assert!(!basic_emoji.contains('\u{0020}'));
    /// assert!(!basic_emoji.contains('\n'));
    /// assert!(basic_emoji.contains('🦃')); // U+1F983 TURKEY
    /// assert!(basic_emoji.contains_str("\u{1F983}"));
    /// assert!(basic_emoji.contains_str("\u{1F6E4}\u{FE0F}")); // railway track
    /// assert!(!basic_emoji.contains_str("\u{0033}\u{FE0F}\u{20E3}"));  // Emoji_Keycap_Sequence, keycap 3
    /// ```
}

#[test]
#[allow(deprecated)]
fn test_to_icu4c_value() {
    // Validate discriminants against PropertyDiscriminants.txt, which is shared with ICU4C.
    for line in include_str!("../tests/data/PropertyDiscriminants.txt").lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(';').map(str::trim);
        let prop = parts.next().unwrap();
        let name = parts.next().unwrap();
        let expected: u16 = parts.next().unwrap().parse().unwrap();
        let actual = match prop.as_bytes() {
            BidiClass::SHORT_NAME => crate::names::PropertyParser::<BidiClass>::new()
                .get_strict(name)
                .unwrap()
                .to_icu4c_value() as u16,
            EastAsianWidth::SHORT_NAME => crate::names::PropertyParser::<EastAsianWidth>::new()
                .get_strict(name)
                .unwrap()
                .to_icu4c_value() as u16,
            GeneralCategory::SHORT_NAME => crate::names::PropertyParser::<GeneralCategory>::new()
                .get_strict(name)
                .unwrap() as u8 as u16,
            GraphemeClusterBreak::SHORT_NAME => {
                crate::names::PropertyParser::<GraphemeClusterBreak>::new()
                    .get_strict(name)
                    .unwrap()
                    .to_icu4c_value() as u16
            }
            HangulSyllableType::SHORT_NAME => {
                crate::names::PropertyParser::<HangulSyllableType>::new()
                    .get_strict(name)
                    .unwrap()
                    .to_icu4c_value() as u16
            }
            IndicConjunctBreak::SHORT_NAME => {
                crate::names::PropertyParser::<IndicConjunctBreak>::new()
                    .get_strict(name)
                    .unwrap()
                    .to_icu4c_value() as u16
            }
            IndicSyllabicCategory::SHORT_NAME => {
                crate::names::PropertyParser::<IndicSyllabicCategory>::new()
                    .get_strict(name)
                    .unwrap()
                    .to_icu4c_value() as u16
            }
            JoiningGroup::SHORT_NAME => crate::names::PropertyParser::<JoiningGroup>::new()
                .get_strict(name)
                .unwrap()
                .to_icu4c_value() as u16,
            JoiningType::SHORT_NAME => crate::names::PropertyParser::<JoiningType>::new()
                .get_strict(name)
                .unwrap()
                .to_icu4c_value() as u16,
            LineBreak::SHORT_NAME => crate::names::PropertyParser::<LineBreak>::new()
                .get_strict(name)
                .unwrap()
                .to_icu4c_value() as u16,
            NumericType::SHORT_NAME => crate::names::PropertyParser::<NumericType>::new()
                .get_strict(name)
                .unwrap()
                .to_icu4c_value() as u16,
            SentenceBreak::SHORT_NAME => crate::names::PropertyParser::<SentenceBreak>::new()
                .get_strict(name)
                .unwrap()
                .to_icu4c_value() as u16,
            VerticalOrientation::SHORT_NAME => {
                crate::names::PropertyParser::<VerticalOrientation>::new()
                    .get_strict(name)
                    .unwrap()
                    .to_icu4c_value() as u16
            }
            WordBreak::SHORT_NAME => crate::names::PropertyParser::<WordBreak>::new()
                .get_strict(name)
                .unwrap()
                .to_icu4c_value() as u16,
            Script::SHORT_NAME => crate::names::PropertyParser::<Script>::new()
                .get_strict(name)
                .unwrap()
                .to_icu4c_value(),
            _ => panic!("Unknown property type: {}", prop),
        };
        assert_eq!(actual, expected);
    }
}

#[test]
fn test_numeric_value() {
    // CCC has UCD-defined numeric values, so validate against those.
    for &value in CanonicalCombiningClass::ALL_VALUES {
        assert_eq!(
            crate::names::PropertyParser::<CanonicalCombiningClass>::new()
                .get_strict(&value.0.to_string()),
            Some(value),
            "{value:?}"
        );
    }
}
