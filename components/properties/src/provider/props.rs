// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Data defintions for CodePointMapData open enums.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>

/// Enumerated property Bidi_Class
///
/// These are the categories required by the Unicode Bidirectional Algorithm.
/// For the property values, see [Bidirectional Class Values](https://unicode.org/reports/tr44/#Bidi_Class_Values).
/// For more information, see [Unicode Standard Annex #9](https://unicode.org/reports/tr41/tr41-28.html#UAX9).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(BidiClassULE)]
pub struct BidiClass(#[doc(hidden)] pub u8);

/// Enumerated property General_Category.
///
/// General_Category specifies the most general classification of a code point, usually
/// determined based on the primary characteristic of the assigned character. For example, is the
/// character a letter, a mark, a number, punctuation, or a symbol, and if so, of what type?
///
/// GeneralCategory only supports specific subcategories (eg `UppercaseLetter`).
/// It does not support grouped categories (eg `Letter`). For grouped categories, use [`GeneralCategoryGroup`](
/// crate::props::GeneralCategoryGroup).
#[derive(Copy, Clone, PartialEq, Eq, Debug, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_enums)] // this type is stable
#[zerovec::make_ule(GeneralCategoryULE)]
#[repr(u8)]
pub enum GeneralCategory {
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

/// Enumerated property Script.
///
/// This is used with both the Script and Script_Extensions Unicode properties.
/// Each character is assigned a single Script, but characters that are used in
/// a particular subset of scripts will be in more than one Script_Extensions set.
/// For example, DEVANAGARI DIGIT NINE has Script=Devanagari, but is also in the
/// Script_Extensions set for Dogra, Kaithi, and Mahajani.
///
/// For more information, see UAX #24: <http://www.unicode.org/reports/tr24/>.
/// See `UScriptCode` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(ScriptULE)]
pub struct Script(pub u16);

/// Enumerated property Hangul_Syllable_Type
///
/// The Unicode standard provides both precomposed Hangul syllables and conjoining Jamo to compose
/// arbitrary Hangul syllables. This property provides that ontology of Hangul code points.
///
/// For more information, see the [Unicode Korean FAQ](https://www.unicode.org/faq/korean.html).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(HangulSyllableTypeULE)]
pub struct HangulSyllableType(#[doc(hidden)] pub u8);

/// Enumerated property East_Asian_Width.
///
/// See "Definition" in UAX #11 for the summary of each property value:
/// <https://www.unicode.org/reports/tr11/#Definitions>
///
/// The numeric value is compatible with `UEastAsianWidth` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(EastAsianWidthULE)]
pub struct EastAsianWidth(#[doc(hidden)] pub u8);

/// Enumerated property Line_Break.
///
/// See "Line Breaking Properties" in UAX #14 for the summary of each property
/// value: <https://www.unicode.org/reports/tr14/#Properties>
///
/// The numeric value is compatible with `ULineBreak` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(LineBreakULE)]
pub struct LineBreak(pub u8);

/// Enumerated property Grapheme_Cluster_Break.
///
/// See "Default Grapheme Cluster Boundary Specification" in UAX #29 for the
/// summary of each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Grapheme_Cluster_Table>
///
/// The numeric value is compatible with `UGraphemeClusterBreak` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // this type is stable
#[repr(transparent)]
#[zerovec::make_ule(GraphemeClusterBreakULE)]
pub struct GraphemeClusterBreak(pub u8);

/// Enumerated property Word_Break.
///
/// See "Default Word Boundary Specification" in UAX #29 for the summary of
/// each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>.
///
/// The numeric value is compatible with `UWordBreakValues` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(WordBreakULE)]
pub struct WordBreak(#[doc(hidden)] pub u8);

/// Enumerated property Sentence_Break.
/// See "Default Sentence Boundary Specification" in UAX #29 for the summary of
/// each property value:
/// <https://www.unicode.org/reports/tr29/#Default_Word_Boundaries>.
///
/// The numeric value is compatible with `USentenceBreak` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(SentenceBreakULE)]
pub struct SentenceBreak(#[doc(hidden)] pub u8);

/// Property Canonical_Combining_Class.
/// See UAX #15:
/// <https://www.unicode.org/reports/tr15/>.
///
/// See `icu::normalizer::properties::CanonicalCombiningClassMap` for the API
/// to look up the Canonical_Combining_Class property by scalar value.
//
// NOTE: The Pernosco debugger has special knowledge
// of this struct. Please do not change the bit layout
// or the crate-module-qualified name of this struct
// without coordination.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(CanonicalCombiningClassULE)]
pub struct CanonicalCombiningClass(#[doc(hidden)] pub u8);

/// Property Indic_Syllabic_Category.
/// See UAX #44:
/// <https://www.unicode.org/reports/tr44/#Indic_Syllabic_Category>.
///
/// The numeric value is compatible with `UIndicSyllabicCategory` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(IndicSyllabicCategoryULE)]
pub struct IndicSyllabicCategory(#[doc(hidden)] pub u8);

/// Enumerated property Joining_Type.
/// See Section 9.2, Arabic Cursive Joining in The Unicode Standard for the summary of
/// each property value.
///
/// The numeric value is compatible with `UJoiningType` in ICU4C.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::provider::props))]
#[allow(clippy::exhaustive_structs)] // newtype
#[repr(transparent)]
#[zerovec::make_ule(JoiningTypeULE)]
pub struct JoiningType(#[doc(hidden)] pub u8);
