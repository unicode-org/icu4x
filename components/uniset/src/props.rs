// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! *** Note: DO NOT USE THESE APIs FOR NOW. ****
//!  Performance improvements and other fixes are still needed on this component.

#![allow(clippy::unreadable_literal, dead_code)]

use crate::enum_props::*;
use crate::provider::*;
use crate::{UnicodeSet, UnicodeSetError};
use icu_provider::prelude::*;
use std::convert::TryInto;

type UnisetResult = Result<UnicodeSet, UnicodeSetError>;

// helper fn
fn get_prop<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(ppucd_provider: &D, resc_key: ResourceKey) -> UnisetResult {
    let data_req = DataRequest {
        resource_path: ResourcePath {
            key: resc_key,
            options: ResourceOptions { variant: None, langid: None },
        },
    };
    let resp: DataResponse<UnicodePropertyMarker> = ppucd_provider.load_payload(&data_req)?;

    let ppucd_property_payload: DataPayload<UnicodePropertyMarker> = resp.take_payload()?;
    let ppucd_property: UnicodePropertyV1 = ppucd_property_payload.get().clone();
    ppucd_property.try_into()
}

//
// Binary property getter fns
//

pub fn get_ascii_hex_digit_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::ASCII_HEX_DIGIT_V1)
}

pub fn get_alnum_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::ALNUM_V1)
}

pub fn get_alphabetic_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::ALPHABETIC_V1)
}

pub fn get_bidi_control_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::BIDI_CONTROL_V1)
}

pub fn get_bidi_mirrored_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::BIDI_MIRRORED_V1)
}

pub fn get_blank_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::BLANK_V1)
}

pub fn get_cased_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::CASED_V1)
}

pub fn get_case_ignorable_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::CASE_IGNORABLE_V1)
}

pub fn get_full_composition_exclusion_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::FULL_COMPOSITION_EXCLUSION_V1)
}

pub fn get_changes_when_casefolded_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::CHANGES_WHEN_CASEFOLDED_V1)
}

pub fn get_changes_when_casemapped_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::CHANGES_WHEN_CASEMAPPED_V1)
}

pub fn get_changes_when_nfkc_casefolded_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::CHANGES_WHEN_NFKC_CASEFOLDED_V1)
}

pub fn get_changes_when_lowercased_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::CHANGES_WHEN_LOWERCASED_V1)
}

pub fn get_changes_when_titlecased_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::CHANGES_WHEN_TITLECASED_V1)
}

pub fn get_changes_when_uppercased_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::CHANGES_WHEN_UPPERCASED_V1)
}

pub fn get_dash_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::DASH_V1)
}

pub fn get_deprecated_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::DEPRECATED_V1)
}

pub fn get_default_ignorable_code_point_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::DEFAULT_IGNORABLE_CODE_POINT_V1)
}

pub fn get_diacritic_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::DIACRITIC_V1)
}

pub fn get_emoji_modifier_base_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::EMOJI_MODIFIER_BASE_V1)
}

pub fn get_emoji_component_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::EMOJI_COMPONENT_V1)
}

pub fn get_emoji_modifier_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::EMOJI_MODIFIER_V1)
}

pub fn get_emoji_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::EMOJI_V1)
}

pub fn get_emoji_presentation_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::EMOJI_PRESENTATION_V1)
}

pub fn get_extender_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::EXTENDER_V1)
}

pub fn get_extended_pictographic_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::EXTENDED_PICTOGRAPHIC_V1)
}

pub fn get_graph_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::GRAPH_V1)
}

pub fn get_grapheme_base_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::GRAPHEME_BASE_V1)
}

pub fn get_grapheme_extend_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::GRAPHEME_EXTEND_V1)
}

pub fn get_grapheme_link_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::GRAPHEME_LINK_V1)
}

pub fn get_hex_digit_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::HEX_DIGIT_V1)
}

pub fn get_hyphen_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::HYPHEN_V1)
}

pub fn get_id_continue_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::ID_CONTINUE_V1)
}

pub fn get_ideographic_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::IDEOGRAPHIC_V1)
}

pub fn get_id_start_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::ID_START_V1)
}

pub fn get_ids_binary_operator_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::IDS_BINARY_OPERATOR_V1)
}

pub fn get_ids_trinary_operator_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::IDS_TRINARY_OPERATOR_V1)
}

pub fn get_join_control_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::JOIN_CONTROL_V1)
}

pub fn get_logical_order_exception_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::LOGICAL_ORDER_EXCEPTION_V1)
}

pub fn get_lowercase_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::LOWERCASE_V1)
}

pub fn get_math_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::MATH_V1)
}

pub fn get_noncharacter_code_point_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::NONCHARACTER_CODE_POINT_V1)
}

pub fn get_nfc_inert_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::NFC_INERT_V1)
}

pub fn get_nfd_inert_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::NFD_INERT_V1)
}

pub fn get_nfkc_inert_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::NFKC_INERT_V1)
}

pub fn get_nfkd_inert_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::NFKD_INERT_V1)
}

pub fn get_pattern_syntax_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::PATTERN_SYNTAX_V1)
}

pub fn get_pattern_white_space_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::PATTERN_WHITE_SPACE_V1)
}

pub fn get_prepended_concatenation_mark_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::PREPENDED_CONCATENATION_MARK_V1)
}

pub fn get_print_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::PRINT_V1)
}

pub fn get_quotation_mark_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::QUOTATION_MARK_V1)
}

pub fn get_radical_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::RADICAL_V1)
}

pub fn get_regional_indicator_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::REGIONAL_INDICATOR_V1)
}

pub fn get_soft_dotted_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::SOFT_DOTTED_V1)
}

pub fn get_segment_starter_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::SEGMENT_STARTER_V1)
}

pub fn get_case_sensitive_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::CASE_SENSITIVE_V1)
}

pub fn get_sentence_terminal_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::SENTENCE_TERMINAL_V1)
}

pub fn get_terminal_punctuation_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::TERMINAL_PUNCTUATION_V1)
}

pub fn get_unified_ideograph_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::UNIFIED_IDEOGRAPH_V1)
}

pub fn get_uppercase_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::UPPERCASE_V1)
}

pub fn get_variation_selector_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::VARIATION_SELECTOR_V1)
}

pub fn get_white_space_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::WHITE_SPACE_V1)
}

pub fn get_xdigit_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::XDIGIT_V1)
}

pub fn get_xid_continue_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::XID_CONTINUE_V1)
}

pub fn get_xid_start_property<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D) -> UnisetResult {
    get_prop(provider, key::XID_START_V1)
}

//
// Enumerated property getter fns
//

/// Return a [`UnicodeSet`] for a particular value of the Bidi_Class Unicode enumerated property
pub fn get_bidi_class_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: BidiClass) -> UnisetResult {
    match enum_val {
        BidiClass::ArabicLetter => get_prop(provider, key::BIDI_CLASS_ARABIC_LETTER_V1),
        BidiClass::ArabicNumber => get_prop(provider, key::BIDI_CLASS_ARABIC_NUMBER_V1),
        BidiClass::ParagraphSeparator => get_prop(provider, key::BIDI_CLASS_PARAGRAPH_SEPARATOR_V1),
        BidiClass::BoundaryNeutral => get_prop(provider, key::BIDI_CLASS_BOUNDARY_NEUTRAL_V1),
        BidiClass::CommonSeparator => get_prop(provider, key::BIDI_CLASS_COMMON_SEPARATOR_V1),
        BidiClass::EuropeanNumber => get_prop(provider, key::BIDI_CLASS_EUROPEAN_NUMBER_V1),
        BidiClass::EuropeanSeparator => get_prop(provider, key::BIDI_CLASS_EUROPEAN_SEPARATOR_V1),
        BidiClass::EuropeanTerminator => get_prop(provider, key::BIDI_CLASS_EUROPEAN_TERMINATOR_V1),
        BidiClass::FirstStrongIsolate => get_prop(provider, key::BIDI_CLASS_FIRST_STRONG_ISOLATE_V1),
        BidiClass::LeftToRight => get_prop(provider, key::BIDI_CLASS_LEFT_TO_RIGHT_V1),
        BidiClass::LeftToRightEmbedding => get_prop(provider, key::BIDI_CLASS_LEFT_TO_RIGHT_EMBEDDING_V1),
        BidiClass::LeftToRightIsolate => get_prop(provider, key::BIDI_CLASS_LEFT_TO_RIGHT_ISOLATE_V1),
        BidiClass::LeftToRightOverride => get_prop(provider, key::BIDI_CLASS_LEFT_TO_RIGHT_OVERRIDE_V1),
        BidiClass::NonspacingMark => get_prop(provider, key::BIDI_CLASS_NONSPACING_MARK_V1),
        BidiClass::OtherNeutral => get_prop(provider, key::BIDI_CLASS_OTHER_NEUTRAL_V1),
        BidiClass::PopDirectionalFormat => get_prop(provider, key::BIDI_CLASS_POP_DIRECTIONAL_FORMAT_V1),
        BidiClass::PopDirectionalIsolate => get_prop(provider, key::BIDI_CLASS_POP_DIRECTIONAL_ISOLATE_V1),
        BidiClass::RightToLeft => get_prop(provider, key::BIDI_CLASS_RIGHT_TO_LEFT_V1),
        BidiClass::RightToLeftEmbedding => get_prop(provider, key::BIDI_CLASS_RIGHT_TO_LEFT_EMBEDDING_V1),
        BidiClass::RightToLeftIsolate => get_prop(provider, key::BIDI_CLASS_RIGHT_TO_LEFT_ISOLATE_V1),
        BidiClass::RightToLeftOverride => get_prop(provider, key::BIDI_CLASS_RIGHT_TO_LEFT_OVERRIDE_V1),
        BidiClass::SegmentSeparator => get_prop(provider, key::BIDI_CLASS_SEGMENT_SEPARATOR_V1),
        BidiClass::WhiteSpace => get_prop(provider, key::BIDI_CLASS_WHITE_SPACE_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Bidi_Paired_Bracket_Type Unicode enumerated property
pub fn get_bidi_paired_bracket_type_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: BidiPairedBracketType) -> UnisetResult {
    match enum_val {
        BidiPairedBracketType::Close => get_prop(provider, key::BIDI_PAIRED_BRACKET_TYPE_CLOSE_V1),
        BidiPairedBracketType::None => get_prop(provider, key::BIDI_PAIRED_BRACKET_TYPE_NONE_V1),
        BidiPairedBracketType::Open => get_prop(provider, key::BIDI_PAIRED_BRACKET_TYPE_OPEN_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Canonical_Combining_Class Unicode enumerated property
pub fn get_canonical_combining_class_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: CanonicalCombiningClass) -> UnisetResult {
    match enum_val {
        CanonicalCombiningClass::NotReordered => get_prop(provider, key::CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1),
        CanonicalCombiningClass::Overlay => get_prop(provider, key::CANONICAL_COMBINING_CLASS_OVERLAY_V1),
        CanonicalCombiningClass::CCC10 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC10_V1),
        CanonicalCombiningClass::CCC103 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC103_V1),
        CanonicalCombiningClass::CCC107 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC107_V1),
        CanonicalCombiningClass::CCC11 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC11_V1),
        CanonicalCombiningClass::CCC118 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC118_V1),
        CanonicalCombiningClass::CCC12 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC12_V1),
        CanonicalCombiningClass::CCC122 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC122_V1),
        CanonicalCombiningClass::CCC129 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC129_V1),
        CanonicalCombiningClass::CCC13 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC13_V1),
        CanonicalCombiningClass::CCC130 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC130_V1),
        CanonicalCombiningClass::CCC132 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC132_V1),
        CanonicalCombiningClass::CCC133 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC133_V1),
        CanonicalCombiningClass::CCC14 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC14_V1),
        CanonicalCombiningClass::CCC15 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC15_V1),
        CanonicalCombiningClass::CCC16 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC16_V1),
        CanonicalCombiningClass::CCC17 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC17_V1),
        CanonicalCombiningClass::CCC18 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC18_V1),
        CanonicalCombiningClass::CCC19 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC19_V1),
        CanonicalCombiningClass::CCC20 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC20_V1),
        CanonicalCombiningClass::AttachedBelowLeft => get_prop(provider, key::CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1),
        CanonicalCombiningClass::AttachedBelow => get_prop(provider, key::CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1),
        CanonicalCombiningClass::CCC21 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC21_V1),
        CanonicalCombiningClass::AttachedAbove => get_prop(provider, key::CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1),
        CanonicalCombiningClass::AttachedAboveRight => get_prop(provider, key::CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1),
        CanonicalCombiningClass::BelowLeft => get_prop(provider, key::CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1),
        CanonicalCombiningClass::CCC22 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC22_V1),
        CanonicalCombiningClass::Below => get_prop(provider, key::CANONICAL_COMBINING_CLASS_BELOW_V1),
        CanonicalCombiningClass::BelowRight => get_prop(provider, key::CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1),
        CanonicalCombiningClass::Left => get_prop(provider, key::CANONICAL_COMBINING_CLASS_LEFT_V1),
        CanonicalCombiningClass::Right => get_prop(provider, key::CANONICAL_COMBINING_CLASS_RIGHT_V1),
        CanonicalCombiningClass::AboveLeft => get_prop(provider, key::CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1),
        CanonicalCombiningClass::CCC23 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC23_V1),
        CanonicalCombiningClass::Above => get_prop(provider, key::CANONICAL_COMBINING_CLASS_ABOVE_V1),
        CanonicalCombiningClass::AboveRight => get_prop(provider, key::CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1),
        CanonicalCombiningClass::DoubleBelow => get_prop(provider, key::CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1),
        CanonicalCombiningClass::DoubleAbove => get_prop(provider, key::CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1),
        CanonicalCombiningClass::CCC24 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC24_V1),
        CanonicalCombiningClass::IotaSubscript => get_prop(provider, key::CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1),
        CanonicalCombiningClass::CCC25 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC25_V1),
        CanonicalCombiningClass::CCC26 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC26_V1),
        CanonicalCombiningClass::CCC27 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC27_V1),
        CanonicalCombiningClass::CCC28 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC28_V1),
        CanonicalCombiningClass::CCC29 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC29_V1),
        CanonicalCombiningClass::CCC30 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC30_V1),
        CanonicalCombiningClass::CCC31 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC31_V1),
        CanonicalCombiningClass::CCC32 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC32_V1),
        CanonicalCombiningClass::CCC33 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC33_V1),
        CanonicalCombiningClass::CCC34 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC34_V1),
        CanonicalCombiningClass::CCC35 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC35_V1),
        CanonicalCombiningClass::CCC36 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC36_V1),
        CanonicalCombiningClass::HanReading => get_prop(provider, key::CANONICAL_COMBINING_CLASS_HAN_READING_V1),
        CanonicalCombiningClass::Nukta => get_prop(provider, key::CANONICAL_COMBINING_CLASS_NUKTA_V1),
        CanonicalCombiningClass::KanaVoicing => get_prop(provider, key::CANONICAL_COMBINING_CLASS_KANA_VOICING_V1),
        CanonicalCombiningClass::CCC84 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC84_V1),
        CanonicalCombiningClass::Virama => get_prop(provider, key::CANONICAL_COMBINING_CLASS_VIRAMA_V1),
        CanonicalCombiningClass::CCC91 => get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC91_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Decomposition_Type Unicode enumerated property
pub fn get_decomposition_type_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: DecompositionType) -> UnisetResult {
    match enum_val {
        DecompositionType::Can => get_prop(provider, key::DECOMPOSITION_TYPE_CAN_V1),
        DecompositionType::Com => get_prop(provider, key::DECOMPOSITION_TYPE_COM_V1),
        DecompositionType::Enc => get_prop(provider, key::DECOMPOSITION_TYPE_ENC_V1),
        DecompositionType::Fin => get_prop(provider, key::DECOMPOSITION_TYPE_FIN_V1),
        DecompositionType::Font => get_prop(provider, key::DECOMPOSITION_TYPE_FONT_V1),
        DecompositionType::Fra => get_prop(provider, key::DECOMPOSITION_TYPE_FRA_V1),
        DecompositionType::Init => get_prop(provider, key::DECOMPOSITION_TYPE_INIT_V1),
        DecompositionType::Iso => get_prop(provider, key::DECOMPOSITION_TYPE_ISO_V1),
        DecompositionType::Med => get_prop(provider, key::DECOMPOSITION_TYPE_MED_V1),
        DecompositionType::Nar => get_prop(provider, key::DECOMPOSITION_TYPE_NAR_V1),
        DecompositionType::Nb => get_prop(provider, key::DECOMPOSITION_TYPE_NB_V1),
        DecompositionType::None => get_prop(provider, key::DECOMPOSITION_TYPE_NONE_V1),
        DecompositionType::Sml => get_prop(provider, key::DECOMPOSITION_TYPE_SML_V1),
        DecompositionType::Sqr => get_prop(provider, key::DECOMPOSITION_TYPE_SQR_V1),
        DecompositionType::Sub => get_prop(provider, key::DECOMPOSITION_TYPE_SUB_V1),
        DecompositionType::Sup => get_prop(provider, key::DECOMPOSITION_TYPE_SUP_V1),
        DecompositionType::Vert => get_prop(provider, key::DECOMPOSITION_TYPE_VERT_V1),
        DecompositionType::Wide => get_prop(provider, key::DECOMPOSITION_TYPE_WIDE_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the East_Asian_Width Unicode enumerated property
pub fn get_east_asian_width_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: EastAsianWidth) -> UnisetResult {
    match enum_val {
        EastAsianWidth::Ambiguous => get_prop(provider, key::EAST_ASIAN_WIDTH_AMBIGUOUS_V1),
        EastAsianWidth::Fullwidth => get_prop(provider, key::EAST_ASIAN_WIDTH_FULLWIDTH_V1),
        EastAsianWidth::Halfwidth => get_prop(provider, key::EAST_ASIAN_WIDTH_HALFWIDTH_V1),
        EastAsianWidth::Neutral => get_prop(provider, key::EAST_ASIAN_WIDTH_NEUTRAL_V1),
        EastAsianWidth::Narrow => get_prop(provider, key::EAST_ASIAN_WIDTH_NARROW_V1),
        EastAsianWidth::Wide => get_prop(provider, key::EAST_ASIAN_WIDTH_WIDE_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the General_Category Unicode enumerated property
pub fn get_general_category_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: GeneralCategory) -> UnisetResult {
    match enum_val {
        GeneralCategory::Other => get_prop(provider, key::GENERAL_CATEGORY_OTHER_V1),
        GeneralCategory::Cntrl => get_prop(provider, key::GENERAL_CATEGORY_CNTRL_V1),
        GeneralCategory::Format => get_prop(provider, key::GENERAL_CATEGORY_FORMAT_V1),
        GeneralCategory::Unassigned => get_prop(provider, key::GENERAL_CATEGORY_UNASSIGNED_V1),
        GeneralCategory::PrivateUse => get_prop(provider, key::GENERAL_CATEGORY_PRIVATE_USE_V1),
        GeneralCategory::Surrogate => get_prop(provider, key::GENERAL_CATEGORY_SURROGATE_V1),
        GeneralCategory::Letter => get_prop(provider, key::GENERAL_CATEGORY_LETTER_V1),
        GeneralCategory::CasedLetter => get_prop(provider, key::GENERAL_CATEGORY_CASED_LETTER_V1),
        GeneralCategory::LowercaseLetter => get_prop(provider, key::GENERAL_CATEGORY_LOWERCASE_LETTER_V1),
        GeneralCategory::ModifierLetter => get_prop(provider, key::GENERAL_CATEGORY_MODIFIER_LETTER_V1),
        GeneralCategory::OtherLetter => get_prop(provider, key::GENERAL_CATEGORY_OTHER_LETTER_V1),
        GeneralCategory::TitlecaseLetter => get_prop(provider, key::GENERAL_CATEGORY_TITLECASE_LETTER_V1),
        GeneralCategory::UppercaseLetter => get_prop(provider, key::GENERAL_CATEGORY_UPPERCASE_LETTER_V1),
        GeneralCategory::CombiningMark => get_prop(provider, key::GENERAL_CATEGORY_COMBINING_MARK_V1),
        GeneralCategory::SpacingMark => get_prop(provider, key::GENERAL_CATEGORY_SPACING_MARK_V1),
        GeneralCategory::EnclosingMark => get_prop(provider, key::GENERAL_CATEGORY_ENCLOSING_MARK_V1),
        GeneralCategory::NonspacingMark => get_prop(provider, key::GENERAL_CATEGORY_NONSPACING_MARK_V1),
        GeneralCategory::Number => get_prop(provider, key::GENERAL_CATEGORY_NUMBER_V1),
        GeneralCategory::Digit => get_prop(provider, key::GENERAL_CATEGORY_DIGIT_V1),
        GeneralCategory::LetterNumber => get_prop(provider, key::GENERAL_CATEGORY_LETTER_NUMBER_V1),
        GeneralCategory::OtherNumber => get_prop(provider, key::GENERAL_CATEGORY_OTHER_NUMBER_V1),
        GeneralCategory::Punct => get_prop(provider, key::GENERAL_CATEGORY_PUNCT_V1),
        GeneralCategory::ConnectorPunctuation => get_prop(provider, key::GENERAL_CATEGORY_CONNECTOR_PUNCTUATION_V1),
        GeneralCategory::DashPunctuation => get_prop(provider, key::GENERAL_CATEGORY_DASH_PUNCTUATION_V1),
        GeneralCategory::ClosePunctuation => get_prop(provider, key::GENERAL_CATEGORY_CLOSE_PUNCTUATION_V1),
        GeneralCategory::FinalPunctuation => get_prop(provider, key::GENERAL_CATEGORY_FINAL_PUNCTUATION_V1),
        GeneralCategory::InitialPunctuation => get_prop(provider, key::GENERAL_CATEGORY_INITIAL_PUNCTUATION_V1),
        GeneralCategory::OtherPunctuation => get_prop(provider, key::GENERAL_CATEGORY_OTHER_PUNCTUATION_V1),
        GeneralCategory::OpenPunctuation => get_prop(provider, key::GENERAL_CATEGORY_OPEN_PUNCTUATION_V1),
        GeneralCategory::Symbol => get_prop(provider, key::GENERAL_CATEGORY_SYMBOL_V1),
        GeneralCategory::CurrencySymbol => get_prop(provider, key::GENERAL_CATEGORY_CURRENCY_SYMBOL_V1),
        GeneralCategory::ModifierSymbol => get_prop(provider, key::GENERAL_CATEGORY_MODIFIER_SYMBOL_V1),
        GeneralCategory::MathSymbol => get_prop(provider, key::GENERAL_CATEGORY_MATH_SYMBOL_V1),
        GeneralCategory::OtherSymbol => get_prop(provider, key::GENERAL_CATEGORY_OTHER_SYMBOL_V1),
        GeneralCategory::Separator => get_prop(provider, key::GENERAL_CATEGORY_SEPARATOR_V1),
        GeneralCategory::LineSeparator => get_prop(provider, key::GENERAL_CATEGORY_LINE_SEPARATOR_V1),
        GeneralCategory::ParagraphSeparator => get_prop(provider, key::GENERAL_CATEGORY_PARAGRAPH_SEPARATOR_V1),
        GeneralCategory::SpaceSeparator => get_prop(provider, key::GENERAL_CATEGORY_SPACE_SEPARATOR_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Grapheme_Cluster_Break Unicode enumerated property
pub fn get_grapheme_cluster_break_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: GraphemeClusterBreak) -> UnisetResult {
    match enum_val {
        GraphemeClusterBreak::Control => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_CONTROL_V1),
        GraphemeClusterBreak::CR => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_CR_V1),
        GraphemeClusterBreak::EBase => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_E_BASE_V1),
        GraphemeClusterBreak::EBaseGAZ => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_E_BASE_GAZ_V1),
        GraphemeClusterBreak::EModifier => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_E_MODIFIER_V1),
        GraphemeClusterBreak::Extend => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_EXTEND_V1),
        GraphemeClusterBreak::GlueAfterZwj => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_GLUE_AFTER_ZWJ_V1),
        GraphemeClusterBreak::L => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_L_V1),
        GraphemeClusterBreak::LF => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_LF_V1),
        GraphemeClusterBreak::LV => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_LV_V1),
        GraphemeClusterBreak::LVT => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_LVT_V1),
        GraphemeClusterBreak::Prepend => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_PREPEND_V1),
        GraphemeClusterBreak::RegionalIndicator => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_REGIONAL_INDICATOR_V1),
        GraphemeClusterBreak::SpacingMark => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_SPACINGMARK_V1),
        GraphemeClusterBreak::T => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_T_V1),
        GraphemeClusterBreak::V => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_V_V1),
        GraphemeClusterBreak::Other => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_OTHER_V1),
        GraphemeClusterBreak::ZWJ => get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_ZWJ_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Hangul_Syllable_Type Unicode enumerated property
pub fn get_hangul_syllable_type_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: HangulSyllableType) -> UnisetResult {
    match enum_val {
        HangulSyllableType::LeadingJamo => get_prop(provider, key::HANGUL_SYLLABLE_TYPE_LEADING_JAMO_V1),
        HangulSyllableType::LVSyllable => get_prop(provider, key::HANGUL_SYLLABLE_TYPE_LV_SYLLABLE_V1),
        HangulSyllableType::LVTSyllable => get_prop(provider, key::HANGUL_SYLLABLE_TYPE_LVT_SYLLABLE_V1),
        HangulSyllableType::NotApplicable => get_prop(provider, key::HANGUL_SYLLABLE_TYPE_NOT_APPLICABLE_V1),
        HangulSyllableType::TrailingJamo => get_prop(provider, key::HANGUL_SYLLABLE_TYPE_TRAILING_JAMO_V1),
        HangulSyllableType::VowelJamo => get_prop(provider, key::HANGUL_SYLLABLE_TYPE_VOWEL_JAMO_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Indic_Positional_Category Unicode enumerated property
pub fn get_indic_positional_category_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: IndicPositionalCategory) -> UnisetResult {
    match enum_val {
        IndicPositionalCategory::Bottom => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_BOTTOM_V1),
        IndicPositionalCategory::BottomAndLeft => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_BOTTOM_AND_LEFT_V1),
        IndicPositionalCategory::BottomAndRight => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_BOTTOM_AND_RIGHT_V1),
        IndicPositionalCategory::Left => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_LEFT_V1),
        IndicPositionalCategory::LeftAndRight => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_LEFT_AND_RIGHT_V1),
        IndicPositionalCategory::NA => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_NA_V1),
        IndicPositionalCategory::Overstruck => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_OVERSTRUCK_V1),
        IndicPositionalCategory::Right => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_RIGHT_V1),
        IndicPositionalCategory::Top => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_V1),
        IndicPositionalCategory::TopAndBottom => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_V1),
        IndicPositionalCategory::TopAndBottomAndLeft => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_AND_LEFT_V1),
        IndicPositionalCategory::TopAndBottomAndRight => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_AND_RIGHT_V1),
        IndicPositionalCategory::TopAndLeft => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_AND_LEFT_V1),
        IndicPositionalCategory::TopAndLeftAndRight => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_AND_LEFT_AND_RIGHT_V1),
        IndicPositionalCategory::TopAndRight => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_AND_RIGHT_V1),
        IndicPositionalCategory::VisualOrderLeft => get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_VISUAL_ORDER_LEFT_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Indic_Syllabic_Category Unicode enumerated property
pub fn get_indic_syllabic_category_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: IndicSyllabicCategory) -> UnisetResult {
    match enum_val {
        IndicSyllabicCategory::Avagraha => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_AVAGRAHA_V1),
        IndicSyllabicCategory::Bindu => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_BINDU_V1),
        IndicSyllabicCategory::BrahmiJoiningNumber => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_BRAHMI_JOINING_NUMBER_V1),
        IndicSyllabicCategory::CantillationMark => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CANTILLATION_MARK_V1),
        IndicSyllabicCategory::Consonant => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_V1),
        IndicSyllabicCategory::ConsonantDead => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_DEAD_V1),
        IndicSyllabicCategory::ConsonantFinal => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_FINAL_V1),
        IndicSyllabicCategory::ConsonantHeadLetter => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_HEAD_LETTER_V1),
        IndicSyllabicCategory::ConsonantInitialPostfixed => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_INITIAL_POSTFIXED_V1),
        IndicSyllabicCategory::ConsonantKiller => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_KILLER_V1),
        IndicSyllabicCategory::ConsonantMedial => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_MEDIAL_V1),
        IndicSyllabicCategory::ConsonantPlaceholder => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_PLACEHOLDER_V1),
        IndicSyllabicCategory::ConsonantPrecedingRepha => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_PRECEDING_REPHA_V1),
        IndicSyllabicCategory::ConsonantPrefixed => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_PREFIXED_V1),
        IndicSyllabicCategory::ConsonantSubjoined => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_SUBJOINED_V1),
        IndicSyllabicCategory::ConsonantSucceedingRepha => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_SUCCEEDING_REPHA_V1),
        IndicSyllabicCategory::ConsonantWithStacker => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_WITH_STACKER_V1),
        IndicSyllabicCategory::GeminationMark => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_GEMINATION_MARK_V1),
        IndicSyllabicCategory::InvisibleStacker => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_INVISIBLE_STACKER_V1),
        IndicSyllabicCategory::Joiner => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_JOINER_V1),
        IndicSyllabicCategory::ModifyingLetter => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_MODIFYING_LETTER_V1),
        IndicSyllabicCategory::NonJoiner => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_NON_JOINER_V1),
        IndicSyllabicCategory::Nukta => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_NUKTA_V1),
        IndicSyllabicCategory::Number => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_NUMBER_V1),
        IndicSyllabicCategory::NumberJoiner => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_NUMBER_JOINER_V1),
        IndicSyllabicCategory::Other => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_OTHER_V1),
        IndicSyllabicCategory::PureKiller => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_PURE_KILLER_V1),
        IndicSyllabicCategory::RegisterShifter => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_REGISTER_SHIFTER_V1),
        IndicSyllabicCategory::SyllableModifier => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_SYLLABLE_MODIFIER_V1),
        IndicSyllabicCategory::ToneLetter => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_TONE_LETTER_V1),
        IndicSyllabicCategory::ToneMark => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_TONE_MARK_V1),
        IndicSyllabicCategory::Virama => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VIRAMA_V1),
        IndicSyllabicCategory::Visarga => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VISARGA_V1),
        IndicSyllabicCategory::Vowel => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VOWEL_V1),
        IndicSyllabicCategory::VowelDependent => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VOWEL_DEPENDENT_V1),
        IndicSyllabicCategory::VowelIndependent => get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VOWEL_INDEPENDENT_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Joining_Group Unicode enumerated property
pub fn get_joining_group_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: JoiningGroup) -> UnisetResult {
    match enum_val {
        JoiningGroup::AfricanFeh => get_prop(provider, key::JOINING_GROUP_AFRICAN_FEH_V1),
        JoiningGroup::AfricanNoon => get_prop(provider, key::JOINING_GROUP_AFRICAN_NOON_V1),
        JoiningGroup::AfricanQaf => get_prop(provider, key::JOINING_GROUP_AFRICAN_QAF_V1),
        JoiningGroup::Ain => get_prop(provider, key::JOINING_GROUP_AIN_V1),
        JoiningGroup::Alaph => get_prop(provider, key::JOINING_GROUP_ALAPH_V1),
        JoiningGroup::Alef => get_prop(provider, key::JOINING_GROUP_ALEF_V1),
        JoiningGroup::Beh => get_prop(provider, key::JOINING_GROUP_BEH_V1),
        JoiningGroup::Beth => get_prop(provider, key::JOINING_GROUP_BETH_V1),
        JoiningGroup::BurushaskiYehBarree => get_prop(provider, key::JOINING_GROUP_BURUSHASKI_YEH_BARREE_V1),
        JoiningGroup::Dal => get_prop(provider, key::JOINING_GROUP_DAL_V1),
        JoiningGroup::DalathRish => get_prop(provider, key::JOINING_GROUP_DALATH_RISH_V1),
        JoiningGroup::E => get_prop(provider, key::JOINING_GROUP_E_V1),
        JoiningGroup::FarsiYeh => get_prop(provider, key::JOINING_GROUP_FARSI_YEH_V1),
        JoiningGroup::Fe => get_prop(provider, key::JOINING_GROUP_FE_V1),
        JoiningGroup::Feh => get_prop(provider, key::JOINING_GROUP_FEH_V1),
        JoiningGroup::FinalSemkath => get_prop(provider, key::JOINING_GROUP_FINAL_SEMKATH_V1),
        JoiningGroup::Gaf => get_prop(provider, key::JOINING_GROUP_GAF_V1),
        JoiningGroup::Gamal => get_prop(provider, key::JOINING_GROUP_GAMAL_V1),
        JoiningGroup::Hah => get_prop(provider, key::JOINING_GROUP_HAH_V1),
        JoiningGroup::HanifiRohingyaKinnaYa => get_prop(provider, key::JOINING_GROUP_HANIFI_ROHINGYA_KINNA_YA_V1),
        JoiningGroup::HanifiRohingyaPa => get_prop(provider, key::JOINING_GROUP_HANIFI_ROHINGYA_PA_V1),
        JoiningGroup::He => get_prop(provider, key::JOINING_GROUP_HE_V1),
        JoiningGroup::Heh => get_prop(provider, key::JOINING_GROUP_HEH_V1),
        JoiningGroup::HehGoal => get_prop(provider, key::JOINING_GROUP_HEH_GOAL_V1),
        JoiningGroup::Heth => get_prop(provider, key::JOINING_GROUP_HETH_V1),
        JoiningGroup::Kaf => get_prop(provider, key::JOINING_GROUP_KAF_V1),
        JoiningGroup::Kaph => get_prop(provider, key::JOINING_GROUP_KAPH_V1),
        JoiningGroup::Khaph => get_prop(provider, key::JOINING_GROUP_KHAPH_V1),
        JoiningGroup::KnottedHeh => get_prop(provider, key::JOINING_GROUP_KNOTTED_HEH_V1),
        JoiningGroup::Lam => get_prop(provider, key::JOINING_GROUP_LAM_V1),
        JoiningGroup::Lamadh => get_prop(provider, key::JOINING_GROUP_LAMADH_V1),
        JoiningGroup::MalayalamBha => get_prop(provider, key::JOINING_GROUP_MALAYALAM_BHA_V1),
        JoiningGroup::MalayalamJa => get_prop(provider, key::JOINING_GROUP_MALAYALAM_JA_V1),
        JoiningGroup::MalayalamLla => get_prop(provider, key::JOINING_GROUP_MALAYALAM_LLA_V1),
        JoiningGroup::MalayalamLlla => get_prop(provider, key::JOINING_GROUP_MALAYALAM_LLLA_V1),
        JoiningGroup::MalayalamNga => get_prop(provider, key::JOINING_GROUP_MALAYALAM_NGA_V1),
        JoiningGroup::MalayalamNna => get_prop(provider, key::JOINING_GROUP_MALAYALAM_NNA_V1),
        JoiningGroup::MalayalamNnna => get_prop(provider, key::JOINING_GROUP_MALAYALAM_NNNA_V1),
        JoiningGroup::MalayalamNya => get_prop(provider, key::JOINING_GROUP_MALAYALAM_NYA_V1),
        JoiningGroup::MalayalamRa => get_prop(provider, key::JOINING_GROUP_MALAYALAM_RA_V1),
        JoiningGroup::MalayalamSsa => get_prop(provider, key::JOINING_GROUP_MALAYALAM_SSA_V1),
        JoiningGroup::MalayalamTta => get_prop(provider, key::JOINING_GROUP_MALAYALAM_TTA_V1),
        JoiningGroup::ManichaeanAleph => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_ALEPH_V1),
        JoiningGroup::ManichaeanAyin => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_AYIN_V1),
        JoiningGroup::ManichaeanBeth => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_BETH_V1),
        JoiningGroup::ManichaeanDaleth => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_DALETH_V1),
        JoiningGroup::ManichaeanDhamedh => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_DHAMEDH_V1),
        JoiningGroup::ManichaeanFive => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_FIVE_V1),
        JoiningGroup::ManichaeanGimel => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_GIMEL_V1),
        JoiningGroup::ManichaeanHeth => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_HETH_V1),
        JoiningGroup::ManichaeanHundred => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_HUNDRED_V1),
        JoiningGroup::ManichaeanKaph => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_KAPH_V1),
        JoiningGroup::ManichaeanLamedh => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_LAMEDH_V1),
        JoiningGroup::ManichaeanMem => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_MEM_V1),
        JoiningGroup::ManichaeanNun => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_NUN_V1),
        JoiningGroup::ManichaeanOne => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_ONE_V1),
        JoiningGroup::ManichaeanPe => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_PE_V1),
        JoiningGroup::ManichaeanQoph => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_QOPH_V1),
        JoiningGroup::ManichaeanResh => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_RESH_V1),
        JoiningGroup::ManichaeanSadhe => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_SADHE_V1),
        JoiningGroup::ManichaeanSamekh => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_SAMEKH_V1),
        JoiningGroup::ManichaeanTaw => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_TAW_V1),
        JoiningGroup::ManichaeanTen => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_TEN_V1),
        JoiningGroup::ManichaeanTeth => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_TETH_V1),
        JoiningGroup::ManichaeanThamedh => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_THAMEDH_V1),
        JoiningGroup::ManichaeanTwenty => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_TWENTY_V1),
        JoiningGroup::ManichaeanWaw => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_WAW_V1),
        JoiningGroup::ManichaeanYodh => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_YODH_V1),
        JoiningGroup::ManichaeanZayin => get_prop(provider, key::JOINING_GROUP_MANICHAEAN_ZAYIN_V1),
        JoiningGroup::Meem => get_prop(provider, key::JOINING_GROUP_MEEM_V1),
        JoiningGroup::Mim => get_prop(provider, key::JOINING_GROUP_MIM_V1),
        JoiningGroup::NoJoiningGroup => get_prop(provider, key::JOINING_GROUP_NO_JOINING_GROUP_V1),
        JoiningGroup::Noon => get_prop(provider, key::JOINING_GROUP_NOON_V1),
        JoiningGroup::Nun => get_prop(provider, key::JOINING_GROUP_NUN_V1),
        JoiningGroup::Nya => get_prop(provider, key::JOINING_GROUP_NYA_V1),
        JoiningGroup::Pe => get_prop(provider, key::JOINING_GROUP_PE_V1),
        JoiningGroup::Qaf => get_prop(provider, key::JOINING_GROUP_QAF_V1),
        JoiningGroup::Qaph => get_prop(provider, key::JOINING_GROUP_QAPH_V1),
        JoiningGroup::Reh => get_prop(provider, key::JOINING_GROUP_REH_V1),
        JoiningGroup::ReversedPe => get_prop(provider, key::JOINING_GROUP_REVERSED_PE_V1),
        JoiningGroup::RohingyaYeh => get_prop(provider, key::JOINING_GROUP_ROHINGYA_YEH_V1),
        JoiningGroup::Sad => get_prop(provider, key::JOINING_GROUP_SAD_V1),
        JoiningGroup::Sadhe => get_prop(provider, key::JOINING_GROUP_SADHE_V1),
        JoiningGroup::Seen => get_prop(provider, key::JOINING_GROUP_SEEN_V1),
        JoiningGroup::Semkath => get_prop(provider, key::JOINING_GROUP_SEMKATH_V1),
        JoiningGroup::Shin => get_prop(provider, key::JOINING_GROUP_SHIN_V1),
        JoiningGroup::StraightWaw => get_prop(provider, key::JOINING_GROUP_STRAIGHT_WAW_V1),
        JoiningGroup::SwashKaf => get_prop(provider, key::JOINING_GROUP_SWASH_KAF_V1),
        JoiningGroup::SyriacWaw => get_prop(provider, key::JOINING_GROUP_SYRIAC_WAW_V1),
        JoiningGroup::Tah => get_prop(provider, key::JOINING_GROUP_TAH_V1),
        JoiningGroup::Taw => get_prop(provider, key::JOINING_GROUP_TAW_V1),
        JoiningGroup::TehMarbuta => get_prop(provider, key::JOINING_GROUP_TEH_MARBUTA_V1),
        JoiningGroup::TehMarbutaGoal => get_prop(provider, key::JOINING_GROUP_HAMZA_ON_HEH_GOAL_V1),
        JoiningGroup::Teth => get_prop(provider, key::JOINING_GROUP_TETH_V1),
        JoiningGroup::Waw => get_prop(provider, key::JOINING_GROUP_WAW_V1),
        JoiningGroup::Yeh => get_prop(provider, key::JOINING_GROUP_YEH_V1),
        JoiningGroup::YehBarree => get_prop(provider, key::JOINING_GROUP_YEH_BARREE_V1),
        JoiningGroup::YehWithTail => get_prop(provider, key::JOINING_GROUP_YEH_WITH_TAIL_V1),
        JoiningGroup::Yudh => get_prop(provider, key::JOINING_GROUP_YUDH_V1),
        JoiningGroup::YudhHe => get_prop(provider, key::JOINING_GROUP_YUDH_HE_V1),
        JoiningGroup::Zain => get_prop(provider, key::JOINING_GROUP_ZAIN_V1),
        JoiningGroup::Zhain => get_prop(provider, key::JOINING_GROUP_ZHAIN_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Joining_Type Unicode enumerated property
pub fn get_joining_type_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: JoiningType) -> UnisetResult {
    match enum_val {
        JoiningType::JoinCausing => get_prop(provider, key::JOINING_TYPE_JOIN_CAUSING_V1),
        JoiningType::DualJoining => get_prop(provider, key::JOINING_TYPE_DUAL_JOINING_V1),
        JoiningType::LeftJoining => get_prop(provider, key::JOINING_TYPE_LEFT_JOINING_V1),
        JoiningType::RightJoining => get_prop(provider, key::JOINING_TYPE_RIGHT_JOINING_V1),
        JoiningType::Transparent => get_prop(provider, key::JOINING_TYPE_TRANSPARENT_V1),
        JoiningType::NonJoining => get_prop(provider, key::JOINING_TYPE_NON_JOINING_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Line_Break Unicode enumerated property
pub fn get_line_break_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: LineBreak) -> UnisetResult {
    match enum_val {
        LineBreak::Ambiguous => get_prop(provider, key::LINE_BREAK_AMBIGUOUS_V1),
        LineBreak::Alphabetic => get_prop(provider, key::LINE_BREAK_ALPHABETIC_V1),
        LineBreak::BreakBoth => get_prop(provider, key::LINE_BREAK_BREAK_BOTH_V1),
        LineBreak::BreakAfter => get_prop(provider, key::LINE_BREAK_BREAK_AFTER_V1),
        LineBreak::BreakBefore => get_prop(provider, key::LINE_BREAK_BREAK_BEFORE_V1),
        LineBreak::MandatoryBreak => get_prop(provider, key::LINE_BREAK_MANDATORY_BREAK_V1),
        LineBreak::ContingentBreak => get_prop(provider, key::LINE_BREAK_CONTINGENT_BREAK_V1),
        LineBreak::ConditionalJapaneseStarter => get_prop(provider, key::LINE_BREAK_CONDITIONAL_JAPANESE_STARTER_V1),
        LineBreak::ClosePunctuation => get_prop(provider, key::LINE_BREAK_CLOSE_PUNCTUATION_V1),
        LineBreak::CombiningMark => get_prop(provider, key::LINE_BREAK_COMBINING_MARK_V1),
        LineBreak::CloseParenthesis => get_prop(provider, key::LINE_BREAK_CLOSE_PARENTHESIS_V1),
        LineBreak::CarriageReturn => get_prop(provider, key::LINE_BREAK_CARRIAGE_RETURN_V1),
        LineBreak::EBase => get_prop(provider, key::LINE_BREAK_E_BASE_V1),
        LineBreak::EModifier => get_prop(provider, key::LINE_BREAK_E_MODIFIER_V1),
        LineBreak::Exclamation => get_prop(provider, key::LINE_BREAK_EXCLAMATION_V1),
        LineBreak::Glue => get_prop(provider, key::LINE_BREAK_GLUE_V1),
        LineBreak::H2 => get_prop(provider, key::LINE_BREAK_H2_V1),
        LineBreak::H3 => get_prop(provider, key::LINE_BREAK_H3_V1),
        LineBreak::HebrewLetter => get_prop(provider, key::LINE_BREAK_HEBREW_LETTER_V1),
        LineBreak::Hyphen => get_prop(provider, key::LINE_BREAK_HYPHEN_V1),
        LineBreak::Ideographic => get_prop(provider, key::LINE_BREAK_IDEOGRAPHIC_V1),
        LineBreak::Inseperable => get_prop(provider, key::LINE_BREAK_INSEPERABLE_V1),
        LineBreak::InfixNumeric => get_prop(provider, key::LINE_BREAK_INFIX_NUMERIC_V1),
        LineBreak::JL => get_prop(provider, key::LINE_BREAK_JL_V1),
        LineBreak::JT => get_prop(provider, key::LINE_BREAK_JT_V1),
        LineBreak::JV => get_prop(provider, key::LINE_BREAK_JV_V1),
        LineBreak::LineFeed => get_prop(provider, key::LINE_BREAK_LINE_FEED_V1),
        LineBreak::NextLine => get_prop(provider, key::LINE_BREAK_NEXT_LINE_V1),
        LineBreak::Nonstarter => get_prop(provider, key::LINE_BREAK_NONSTARTER_V1),
        LineBreak::Numeric => get_prop(provider, key::LINE_BREAK_NUMERIC_V1),
        LineBreak::OpenPunctuation => get_prop(provider, key::LINE_BREAK_OPEN_PUNCTUATION_V1),
        LineBreak::PostfixNumeric => get_prop(provider, key::LINE_BREAK_POSTFIX_NUMERIC_V1),
        LineBreak::PrefixNumeric => get_prop(provider, key::LINE_BREAK_PREFIX_NUMERIC_V1),
        LineBreak::Quotation => get_prop(provider, key::LINE_BREAK_QUOTATION_V1),
        LineBreak::RegionalIndicator => get_prop(provider, key::LINE_BREAK_REGIONAL_INDICATOR_V1),
        LineBreak::ComplexContext => get_prop(provider, key::LINE_BREAK_COMPLEX_CONTEXT_V1),
        LineBreak::Surrogate => get_prop(provider, key::LINE_BREAK_SURROGATE_V1),
        LineBreak::Space => get_prop(provider, key::LINE_BREAK_SPACE_V1),
        LineBreak::BreakSymbols => get_prop(provider, key::LINE_BREAK_BREAK_SYMBOLS_V1),
        LineBreak::WordJoiner => get_prop(provider, key::LINE_BREAK_WORD_JOINER_V1),
        LineBreak::Unknown => get_prop(provider, key::LINE_BREAK_UNKNOWN_V1),
        LineBreak::ZWSpace => get_prop(provider, key::LINE_BREAK_ZWSPACE_V1),
        LineBreak::ZWJ => get_prop(provider, key::LINE_BREAK_ZWJ_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Lead_Canonical_Combining_Class Unicode enumerated property
pub fn get_lead_canonical_combining_class_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: LeadCanonicalCombiningClass) -> UnisetResult {
    match enum_val {
        LeadCanonicalCombiningClass::NotReordered => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1),
        LeadCanonicalCombiningClass::Overlay => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_OVERLAY_V1),
        LeadCanonicalCombiningClass::CCC10 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC10_V1),
        LeadCanonicalCombiningClass::CCC103 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC103_V1),
        LeadCanonicalCombiningClass::CCC107 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC107_V1),
        LeadCanonicalCombiningClass::CCC11 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC11_V1),
        LeadCanonicalCombiningClass::CCC118 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC118_V1),
        LeadCanonicalCombiningClass::CCC12 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC12_V1),
        LeadCanonicalCombiningClass::CCC122 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC122_V1),
        LeadCanonicalCombiningClass::CCC129 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC129_V1),
        LeadCanonicalCombiningClass::CCC13 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC13_V1),
        LeadCanonicalCombiningClass::CCC130 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC130_V1),
        LeadCanonicalCombiningClass::CCC132 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC132_V1),
        LeadCanonicalCombiningClass::CCC133 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC133_V1),
        LeadCanonicalCombiningClass::CCC14 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC14_V1),
        LeadCanonicalCombiningClass::CCC15 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC15_V1),
        LeadCanonicalCombiningClass::CCC16 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC16_V1),
        LeadCanonicalCombiningClass::CCC17 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC17_V1),
        LeadCanonicalCombiningClass::CCC18 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC18_V1),
        LeadCanonicalCombiningClass::CCC19 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC19_V1),
        LeadCanonicalCombiningClass::CCC20 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC20_V1),
        LeadCanonicalCombiningClass::AttachedBelowLeft => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1),
        LeadCanonicalCombiningClass::AttachedBelow => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1),
        LeadCanonicalCombiningClass::CCC21 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC21_V1),
        LeadCanonicalCombiningClass::AttachedAbove => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1),
        LeadCanonicalCombiningClass::AttachedAboveRight => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1),
        LeadCanonicalCombiningClass::BelowLeft => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1),
        LeadCanonicalCombiningClass::CCC22 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC22_V1),
        LeadCanonicalCombiningClass::Below => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_BELOW_V1),
        LeadCanonicalCombiningClass::BelowRight => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1),
        LeadCanonicalCombiningClass::Left => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_LEFT_V1),
        LeadCanonicalCombiningClass::Right => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_RIGHT_V1),
        LeadCanonicalCombiningClass::AboveLeft => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1),
        LeadCanonicalCombiningClass::CCC23 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC23_V1),
        LeadCanonicalCombiningClass::Above => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ABOVE_V1),
        LeadCanonicalCombiningClass::AboveRight => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1),
        LeadCanonicalCombiningClass::DoubleBelow => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1),
        LeadCanonicalCombiningClass::DoubleAbove => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1),
        LeadCanonicalCombiningClass::CCC24 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC24_V1),
        LeadCanonicalCombiningClass::IotaSubscript => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1),
        LeadCanonicalCombiningClass::CCC25 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC25_V1),
        LeadCanonicalCombiningClass::CCC26 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC26_V1),
        LeadCanonicalCombiningClass::CCC27 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC27_V1),
        LeadCanonicalCombiningClass::CCC28 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC28_V1),
        LeadCanonicalCombiningClass::CCC29 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC29_V1),
        LeadCanonicalCombiningClass::CCC30 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC30_V1),
        LeadCanonicalCombiningClass::CCC31 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC31_V1),
        LeadCanonicalCombiningClass::CCC32 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC32_V1),
        LeadCanonicalCombiningClass::CCC33 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC33_V1),
        LeadCanonicalCombiningClass::CCC34 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC34_V1),
        LeadCanonicalCombiningClass::CCC35 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC35_V1),
        LeadCanonicalCombiningClass::CCC36 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC36_V1),
        LeadCanonicalCombiningClass::HanReading => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_HAN_READING_V1),
        LeadCanonicalCombiningClass::Nukta => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_NUKTA_V1),
        LeadCanonicalCombiningClass::KanaVoicing => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_KANA_VOICING_V1),
        LeadCanonicalCombiningClass::CCC84 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC84_V1),
        LeadCanonicalCombiningClass::Virama => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_VIRAMA_V1),
        LeadCanonicalCombiningClass::CCC91 => get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC91_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the NFC_Quick_Check Unicode enumerated property
pub fn get_nfc_quick_check_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: NFCQuickCheck) -> UnisetResult {
    match enum_val {
        NFCQuickCheck::Maybe => get_prop(provider, key::NFC_QUICK_CHECK_MAYBE_V1),
        NFCQuickCheck::No => get_prop(provider, key::NFC_QUICK_CHECK_NO_V1),
        NFCQuickCheck::Yes => get_prop(provider, key::NFC_QUICK_CHECK_YES_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the NFD_Quick_Check Unicode enumerated property
pub fn get_nfd_quick_check_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: NFDQuickCheck) -> UnisetResult {
    match enum_val {
        NFDQuickCheck::No => get_prop(provider, key::NFD_QUICK_CHECK_NO_V1),
        NFDQuickCheck::Yes => get_prop(provider, key::NFD_QUICK_CHECK_YES_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the NFKC_Quick_Check Unicode enumerated property
pub fn get_nfkc_quick_check_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: NFKCQuickCheck) -> UnisetResult {
    match enum_val {
        NFKCQuickCheck::Maybe => get_prop(provider, key::NFKC_QUICK_CHECK_MAYBE_V1),
        NFKCQuickCheck::No => get_prop(provider, key::NFKC_QUICK_CHECK_NO_V1),
        NFKCQuickCheck::Yes => get_prop(provider, key::NFKC_QUICK_CHECK_YES_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the NFKD_Quick_Check Unicode enumerated property
pub fn get_nfkd_quick_check_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: NFKDQuickCheck) -> UnisetResult {
    match enum_val {
        NFKDQuickCheck::No => get_prop(provider, key::NFKD_QUICK_CHECK_NO_V1),
        NFKDQuickCheck::Yes => get_prop(provider, key::NFKD_QUICK_CHECK_YES_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Numeric_Type Unicode enumerated property
pub fn get_numeric_type_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: NumericType) -> UnisetResult {
    match enum_val {
        NumericType::Decimal => get_prop(provider, key::NUMERIC_TYPE_DECIMAL_V1),
        NumericType::Digit => get_prop(provider, key::NUMERIC_TYPE_DIGIT_V1),
        NumericType::None => get_prop(provider, key::NUMERIC_TYPE_NONE_V1),
        NumericType::Numeric => get_prop(provider, key::NUMERIC_TYPE_NUMERIC_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Sentence_Break Unicode enumerated property
pub fn get_sentence_break_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: SentenceBreak) -> UnisetResult {
    match enum_val {
        SentenceBreak::ATerm => get_prop(provider, key::SENTENCE_BREAK_ATERM_V1),
        SentenceBreak::Close => get_prop(provider, key::SENTENCE_BREAK_CLOSE_V1),
        SentenceBreak::CR => get_prop(provider, key::SENTENCE_BREAK_CR_V1),
        SentenceBreak::Extend => get_prop(provider, key::SENTENCE_BREAK_EXTEND_V1),
        SentenceBreak::Format => get_prop(provider, key::SENTENCE_BREAK_FORMAT_V1),
        SentenceBreak::OLetter => get_prop(provider, key::SENTENCE_BREAK_OLETTER_V1),
        SentenceBreak::LF => get_prop(provider, key::SENTENCE_BREAK_LF_V1),
        SentenceBreak::Lower => get_prop(provider, key::SENTENCE_BREAK_LOWER_V1),
        SentenceBreak::Numeric => get_prop(provider, key::SENTENCE_BREAK_NUMERIC_V1),
        SentenceBreak::SContinue => get_prop(provider, key::SENTENCE_BREAK_SCONTINUE_V1),
        SentenceBreak::Sep => get_prop(provider, key::SENTENCE_BREAK_SEP_V1),
        SentenceBreak::Sp => get_prop(provider, key::SENTENCE_BREAK_SP_V1),
        SentenceBreak::STerm => get_prop(provider, key::SENTENCE_BREAK_STERM_V1),
        SentenceBreak::Upper => get_prop(provider, key::SENTENCE_BREAK_UPPER_V1),
        SentenceBreak::Other => get_prop(provider, key::SENTENCE_BREAK_OTHER_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Trail_Canonical_Combining_Class Unicode enumerated property
pub fn get_trail_canonical_combining_class_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: TrailCanonicalCombiningClass) -> UnisetResult {
    match enum_val {
        TrailCanonicalCombiningClass::NotReordered => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1),
        TrailCanonicalCombiningClass::Overlay => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_OVERLAY_V1),
        TrailCanonicalCombiningClass::CCC10 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC10_V1),
        TrailCanonicalCombiningClass::CCC103 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC103_V1),
        TrailCanonicalCombiningClass::CCC107 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC107_V1),
        TrailCanonicalCombiningClass::CCC11 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC11_V1),
        TrailCanonicalCombiningClass::CCC118 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC118_V1),
        TrailCanonicalCombiningClass::CCC12 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC12_V1),
        TrailCanonicalCombiningClass::CCC122 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC122_V1),
        TrailCanonicalCombiningClass::CCC129 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC129_V1),
        TrailCanonicalCombiningClass::CCC13 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC13_V1),
        TrailCanonicalCombiningClass::CCC130 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC130_V1),
        TrailCanonicalCombiningClass::CCC132 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC132_V1),
        TrailCanonicalCombiningClass::CCC133 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC133_V1),
        TrailCanonicalCombiningClass::CCC14 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC14_V1),
        TrailCanonicalCombiningClass::CCC15 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC15_V1),
        TrailCanonicalCombiningClass::CCC16 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC16_V1),
        TrailCanonicalCombiningClass::CCC17 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC17_V1),
        TrailCanonicalCombiningClass::CCC18 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC18_V1),
        TrailCanonicalCombiningClass::CCC19 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC19_V1),
        TrailCanonicalCombiningClass::CCC20 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC20_V1),
        TrailCanonicalCombiningClass::AttachedBelowLeft => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1),
        TrailCanonicalCombiningClass::AttachedBelow => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1),
        TrailCanonicalCombiningClass::CCC21 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC21_V1),
        TrailCanonicalCombiningClass::AttachedAbove => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1),
        TrailCanonicalCombiningClass::AttachedAboveRight => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1),
        TrailCanonicalCombiningClass::BelowLeft => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1),
        TrailCanonicalCombiningClass::CCC22 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC22_V1),
        TrailCanonicalCombiningClass::Below => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_BELOW_V1),
        TrailCanonicalCombiningClass::BelowRight => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1),
        TrailCanonicalCombiningClass::Left => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_LEFT_V1),
        TrailCanonicalCombiningClass::Right => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_RIGHT_V1),
        TrailCanonicalCombiningClass::AboveLeft => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1),
        TrailCanonicalCombiningClass::CCC23 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC23_V1),
        TrailCanonicalCombiningClass::Above => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_V1),
        TrailCanonicalCombiningClass::AboveRight => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1),
        TrailCanonicalCombiningClass::DoubleBelow => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1),
        TrailCanonicalCombiningClass::DoubleAbove => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1),
        TrailCanonicalCombiningClass::CCC24 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC24_V1),
        TrailCanonicalCombiningClass::IotaSubscript => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1),
        TrailCanonicalCombiningClass::CCC25 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC25_V1),
        TrailCanonicalCombiningClass::CCC26 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC26_V1),
        TrailCanonicalCombiningClass::CCC27 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC27_V1),
        TrailCanonicalCombiningClass::CCC28 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC28_V1),
        TrailCanonicalCombiningClass::CCC29 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC29_V1),
        TrailCanonicalCombiningClass::CCC30 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC30_V1),
        TrailCanonicalCombiningClass::CCC31 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC31_V1),
        TrailCanonicalCombiningClass::CCC32 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC32_V1),
        TrailCanonicalCombiningClass::CCC33 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC33_V1),
        TrailCanonicalCombiningClass::CCC34 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC34_V1),
        TrailCanonicalCombiningClass::CCC35 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC35_V1),
        TrailCanonicalCombiningClass::CCC36 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC36_V1),
        TrailCanonicalCombiningClass::HanReading => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_HAN_READING_V1),
        TrailCanonicalCombiningClass::Nukta => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_NUKTA_V1),
        TrailCanonicalCombiningClass::KanaVoicing => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_KANA_VOICING_V1),
        TrailCanonicalCombiningClass::CCC84 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC84_V1),
        TrailCanonicalCombiningClass::Virama => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_VIRAMA_V1),
        TrailCanonicalCombiningClass::CCC91 => get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC91_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Vertical_Orientation Unicode enumerated property
pub fn get_vertical_orientation_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: VerticalOrientation) -> UnisetResult {
    match enum_val {
        VerticalOrientation::Rotated => get_prop(provider, key::VERTICAL_ORIENTATION_ROTATED_V1),
        VerticalOrientation::TransformedRotated => get_prop(provider, key::VERTICAL_ORIENTATION_TRANSFORMED_ROTATED_V1),
        VerticalOrientation::TransformedUpright => get_prop(provider, key::VERTICAL_ORIENTATION_TRANSFORMED_UPRIGHT_V1),
        VerticalOrientation::Upright => get_prop(provider, key::VERTICAL_ORIENTATION_UPRIGHT_V1),
    }
}

/// Return a [`UnicodeSet`] for a particular value of the Word_Break Unicode enumerated property
pub fn get_word_break_val_set<'d, D: DataProvider<'d, 'd, UnicodePropertyMarker> + ?Sized>(provider: &D, enum_val: WordBreak) -> UnisetResult {
    match enum_val {
        WordBreak::CR => get_prop(provider, key::WORD_BREAK_CR_V1),
        WordBreak::DoubleQuote => get_prop(provider, key::WORD_BREAK_DOUBLE_QUOTE_V1),
        WordBreak::EBase => get_prop(provider, key::WORD_BREAK_E_BASE_V1),
        WordBreak::EBaseGAZ => get_prop(provider, key::WORD_BREAK_E_BASE_GAZ_V1),
        WordBreak::EModifier => get_prop(provider, key::WORD_BREAK_E_MODIFIER_V1),
        WordBreak::ExtendNumLet => get_prop(provider, key::WORD_BREAK_EXTENDNUMLET_V1),
        WordBreak::Extend => get_prop(provider, key::WORD_BREAK_EXTEND_V1),
        WordBreak::Format => get_prop(provider, key::WORD_BREAK_FORMAT_V1),
        WordBreak::GlueAfterZwj => get_prop(provider, key::WORD_BREAK_GLUE_AFTER_ZWJ_V1),
        WordBreak::HebrewLetter => get_prop(provider, key::WORD_BREAK_HEBREW_LETTER_V1),
        WordBreak::Katakana => get_prop(provider, key::WORD_BREAK_KATAKANA_V1),
        WordBreak::ALetter => get_prop(provider, key::WORD_BREAK_ALETTER_V1),
        WordBreak::LF => get_prop(provider, key::WORD_BREAK_LF_V1),
        WordBreak::MidNumLet => get_prop(provider, key::WORD_BREAK_MIDNUMLET_V1),
        WordBreak::MidLetter => get_prop(provider, key::WORD_BREAK_MIDLETTER_V1),
        WordBreak::MidNum => get_prop(provider, key::WORD_BREAK_MIDNUM_V1),
        WordBreak::Newline => get_prop(provider, key::WORD_BREAK_NEWLINE_V1),
        WordBreak::Numeric => get_prop(provider, key::WORD_BREAK_NUMERIC_V1),
        WordBreak::RegionalIndicator => get_prop(provider, key::WORD_BREAK_REGIONAL_INDICATOR_V1),
        WordBreak::SingleQuote => get_prop(provider, key::WORD_BREAK_SINGLE_QUOTE_V1),
        WordBreak::WSegSpace => get_prop(provider, key::WORD_BREAK_WSEGSPACE_V1),
        WordBreak::Other => get_prop(provider, key::WORD_BREAK_OTHER_V1),
        WordBreak::ZWJ => get_prop(provider, key::WORD_BREAK_ZWJ_V1),
    }
}
