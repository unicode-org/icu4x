// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Definitions of [Unicode Properties] and APIs for
//! retrieving property data in an appropriate data structure.
//!
//! This module is published as its own crate ([`icu_properties`](https://docs.rs/icu_properties/latest/icu_properties/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! APIs that return a [`CodePointSetData`] exist for binary properties and certain enumerated
//! properties.
//!
//! APIs that return a [`CodePointMapData`] exist for certain enumerated properties.
//!
//! # Examples
//!
//! ## Property data as `CodePointSetData`s
//!
//! ```
//! use icu::properties::{CodePointSetData, CodePointMapData};
//! use icu::properties::props::{GeneralCategory, Emoji};
//!
//! // A binary property as a `CodePointSetData`
//!
//! assert!(CodePointSetData::new::<Emoji>().contains('🎃')); // U+1F383 JACK-O-LANTERN
//! assert!(!CodePointSetData::new::<Emoji>().contains('木')); // U+6728
//!
//! // An individual enumerated property value as a `CodePointSetData`
//!
//! let line_sep_data = CodePointMapData::<GeneralCategory>::new()
//!     .get_set_for_value(GeneralCategory::LineSeparator);
//! let line_sep = line_sep_data.as_borrowed();
//!
//! assert!(line_sep.contains32(0x2028));
//! assert!(!line_sep.contains32(0x2029));
//! ```
//!
//! ## Property data as `CodePointMapData`s
//!
//! ```
//! use icu::properties::CodePointMapData;
//! use icu::properties::props::Script;
//!
//! assert_eq!(CodePointMapData::<Script>::new().get('🎃'), Script::Common); // U+1F383 JACK-O-LANTERN
//! assert_eq!(CodePointMapData::<Script>::new().get('木'), Script::Han); // U+6728
//! ```
//!
//! [`ICU4X`]: ../icu/index.html
//! [Unicode Properties]: https://unicode-org.github.io/icu/userguide/strings/properties.html
//! [`CodePointSetData`]: crate::CodePointSetData
//! [`CodePointMapData`]: crate::CodePointMapData

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

#[cfg(feature = "bidi")]
mod bidi;

mod code_point_set;
pub use code_point_set::{CodePointSetData, CodePointSetDataBorrowed};
mod code_point_map;
pub use code_point_map::{CodePointMapData, CodePointMapDataBorrowed};
mod unicode_set;
pub use unicode_set::{UnicodeSetData, UnicodeSetDataBorrowed};
mod names;
pub use names::{
    PropertyNames, PropertyNamesBorrowed, PropertyParser, PropertyParserBorrowed, ScriptMapper,
    ScriptMapperBorrowed,
};

// NOTE: The Pernosco debugger has special knowledge
// of the `CanonicalCombiningClass` struct inside the `props`
// module. Please do not change the crate-module-qualified
// name of that struct without coordination.
pub mod props;

pub mod bidi_data;
pub mod provider;
mod runtime;
pub use runtime::UnicodeProperty;

#[allow(clippy::exhaustive_structs)] // TODO
pub mod script;
mod trievalue;

mod private {
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_general_category() {
        use icu::properties::props::GeneralCategory;
        use icu::properties::props::GeneralCategoryGroup;
        use icu::properties::CodePointMapData;

        let digits_data = CodePointMapData::<GeneralCategory>::new()
            .get_set_for_value_group(GeneralCategoryGroup::Number);
        let digits = digits_data.as_borrowed();

        assert!(digits.contains('5'));
        assert!(digits.contains('\u{0665}')); // U+0665 ARABIC-INDIC DIGIT FIVE
        assert!(digits.contains('\u{096b}')); // U+0969 DEVANAGARI DIGIT FIVE

        assert!(!digits.contains('A'));
    }

    #[test]
    fn test_script() {
        use icu::properties::props::Script;
        use icu::properties::CodePointMapData;

        let thai_data = CodePointMapData::<Script>::new().get_set_for_value(Script::Thai);
        let thai = thai_data.as_borrowed();

        assert!(thai.contains('\u{0e01}')); // U+0E01 THAI CHARACTER KO KAI
        assert!(thai.contains('\u{0e50}')); // U+0E50 THAI DIGIT ZERO

        assert!(!thai.contains('A'));
        assert!(!thai.contains('\u{0e3f}')); // U+0E50 THAI CURRENCY SYMBOL BAHT
    }

    #[test]
    fn test_gc_groupings() {
        use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
        use icu::properties::CodePointMapData;
        use icu_collections::codepointinvlist::CodePointInversionListBuilder;

        let test_group = |category: GeneralCategoryGroup, subcategories: &[GeneralCategory]| {
            let category_set =
                CodePointMapData::<GeneralCategory>::new().get_set_for_value_group(category);
            let category_set = category_set
                .as_code_point_inversion_list()
                .expect("The data should be valid");

            let mut builder = CodePointInversionListBuilder::new();
            for &subcategory in subcategories {
                let gc_set_data =
                    CodePointMapData::<GeneralCategory>::new().get_set_for_value(subcategory);
                let gc_set = gc_set_data.as_borrowed();
                for range in gc_set.iter_ranges() {
                    builder.add_range32(range);
                }
            }
            let combined_set = builder.build();
            println!("{category:?} {subcategories:?}");
            assert_eq!(
                category_set.get_inversion_list_vec(),
                combined_set.get_inversion_list_vec()
            );
        };

        test_group(
            GeneralCategoryGroup::Letter,
            &[
                GeneralCategory::UppercaseLetter,
                GeneralCategory::LowercaseLetter,
                GeneralCategory::TitlecaseLetter,
                GeneralCategory::ModifierLetter,
                GeneralCategory::OtherLetter,
            ],
        );
        test_group(
            GeneralCategoryGroup::Other,
            &[
                GeneralCategory::Control,
                GeneralCategory::Format,
                GeneralCategory::Unassigned,
                GeneralCategory::PrivateUse,
                GeneralCategory::Surrogate,
            ],
        );
        test_group(
            GeneralCategoryGroup::Mark,
            &[
                GeneralCategory::SpacingMark,
                GeneralCategory::EnclosingMark,
                GeneralCategory::NonspacingMark,
            ],
        );
        test_group(
            GeneralCategoryGroup::Number,
            &[
                GeneralCategory::DecimalNumber,
                GeneralCategory::LetterNumber,
                GeneralCategory::OtherNumber,
            ],
        );
        test_group(
            GeneralCategoryGroup::Punctuation,
            &[
                GeneralCategory::ConnectorPunctuation,
                GeneralCategory::DashPunctuation,
                GeneralCategory::ClosePunctuation,
                GeneralCategory::FinalPunctuation,
                GeneralCategory::InitialPunctuation,
                GeneralCategory::OtherPunctuation,
                GeneralCategory::OpenPunctuation,
            ],
        );
        test_group(
            GeneralCategoryGroup::Symbol,
            &[
                GeneralCategory::CurrencySymbol,
                GeneralCategory::ModifierSymbol,
                GeneralCategory::MathSymbol,
                GeneralCategory::OtherSymbol,
            ],
        );
        test_group(
            GeneralCategoryGroup::Separator,
            &[
                GeneralCategory::LineSeparator,
                GeneralCategory::ParagraphSeparator,
                GeneralCategory::SpaceSeparator,
            ],
        );
    }

    #[test]
    fn test_gc_surrogate() {
        use icu::properties::props::GeneralCategory;
        use icu::properties::CodePointMapData;

        let surrogates_data = CodePointMapData::<GeneralCategory>::new()
            .get_set_for_value(GeneralCategory::Surrogate);
        let surrogates = surrogates_data.as_borrowed();

        assert!(surrogates.contains32(0xd800));
        assert!(surrogates.contains32(0xd900));
        assert!(surrogates.contains32(0xdfff));

        assert!(!surrogates.contains('A'));
    }
}
