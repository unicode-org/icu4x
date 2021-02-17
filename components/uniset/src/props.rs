// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
#![allow(clippy::unreadable_literal, dead_code)]

use crate::provider::*;
use crate::{UnicodeSet, UnicodeSetError};
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::convert::TryInto;

// helper fn
fn get_prop<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    ppucd_provider: &D,
    resc_key: ResourceKey,
) -> Result<UnicodeSet, UnicodeSetError> {
    let data_req = DataRequest {
        resource_path: ResourcePath {
            key: resc_key,
            options: ResourceOptions {
                variant: None,
                langid: None,
            },
        },
    };
    let mut resp: DataResponse<UnicodeProperty> = ppucd_provider.load_payload(&data_req)?;

    let ppucd_property_cow: Cow<UnicodeProperty> = resp.take_payload()?;
    let ppucd_property: UnicodeProperty = ppucd_property_cow.into_owned();
    ppucd_property.try_into()
}

//
// Binary property getter fns
//

pub fn get_ascii_hex_digit_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::ASCII_HEX_DIGIT_V1)
}

pub fn get_alnum_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::ALNUM_V1)
}

pub fn get_alphabetic_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::ALPHABETIC_V1)
}

pub fn get_bidi_control_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CONTROL_V1)
}

pub fn get_bidi_mirrored_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_MIRRORED_V1)
}

pub fn get_blank_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BLANK_V1)
}

pub fn get_cased_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CASED_V1)
}

pub fn get_case_ignorable_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CASE_IGNORABLE_V1)
}

pub fn get_full_composition_exclusion_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::FULL_COMPOSITION_EXCLUSION_V1)
}

pub fn get_changes_when_casefolded_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CHANGES_WHEN_CASEFOLDED_V1)
}

pub fn get_changes_when_casemapped_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CHANGES_WHEN_CASEMAPPED_V1)
}

pub fn get_changes_when_nfkc_casefolded_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CHANGES_WHEN_NFKC_CASEFOLDED_V1)
}

pub fn get_changes_when_lowercased_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CHANGES_WHEN_LOWERCASED_V1)
}

pub fn get_changes_when_titlecased_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CHANGES_WHEN_TITLECASED_V1)
}

pub fn get_changes_when_uppercased_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CHANGES_WHEN_UPPERCASED_V1)
}

pub fn get_dash_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DASH_V1)
}

pub fn get_deprecated_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DEPRECATED_V1)
}

pub fn get_default_ignorable_code_point_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DEFAULT_IGNORABLE_CODE_POINT_V1)
}

pub fn get_diacritic_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DIACRITIC_V1)
}

pub fn get_emoji_modifier_base_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EMOJI_MODIFIER_BASE_V1)
}

pub fn get_emoji_component_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EMOJI_COMPONENT_V1)
}

pub fn get_emoji_modifier_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EMOJI_MODIFIER_V1)
}

pub fn get_emoji_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EMOJI_V1)
}

pub fn get_emoji_presentation_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EMOJI_PRESENTATION_V1)
}

pub fn get_extender_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EXTENDER_V1)
}

pub fn get_extended_pictographic_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EXTENDED_PICTOGRAPHIC_V1)
}

pub fn get_graph_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPH_V1)
}

pub fn get_grapheme_base_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_BASE_V1)
}

pub fn get_grapheme_extend_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_EXTEND_V1)
}

pub fn get_grapheme_link_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_LINK_V1)
}

pub fn get_hex_digit_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HEX_DIGIT_V1)
}

pub fn get_hyphen_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HYPHEN_V1)
}

pub fn get_id_continue_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::ID_CONTINUE_V1)
}

pub fn get_ideographic_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::IDEOGRAPHIC_V1)
}

pub fn get_id_start_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::ID_START_V1)
}

pub fn get_ids_binary_operator_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::IDS_BINARY_OPERATOR_V1)
}

pub fn get_ids_trinary_operator_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::IDS_TRINARY_OPERATOR_V1)
}

pub fn get_join_control_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOIN_CONTROL_V1)
}

pub fn get_logical_order_exception_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LOGICAL_ORDER_EXCEPTION_V1)
}

pub fn get_lowercase_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LOWERCASE_V1)
}

pub fn get_math_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::MATH_V1)
}

pub fn get_noncharacter_code_point_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NONCHARACTER_CODE_POINT_V1)
}

pub fn get_nfc_inert_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFC_INERT_V1)
}

pub fn get_nfd_inert_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFD_INERT_V1)
}

pub fn get_nfkc_inert_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFKC_INERT_V1)
}

pub fn get_nfkd_inert_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFKD_INERT_V1)
}

pub fn get_pattern_syntax_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::PATTERN_SYNTAX_V1)
}

pub fn get_pattern_white_space_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::PATTERN_WHITE_SPACE_V1)
}

pub fn get_prepended_concatenation_mark_property<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::PREPENDED_CONCATENATION_MARK_V1)
}

pub fn get_print_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::PRINT_V1)
}

pub fn get_quotation_mark_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::QUOTATION_MARK_V1)
}

pub fn get_radical_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::RADICAL_V1)
}

pub fn get_regional_indicator_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::REGIONAL_INDICATOR_V1)
}

pub fn get_soft_dotted_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SOFT_DOTTED_V1)
}

pub fn get_segment_starter_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SEGMENT_STARTER_V1)
}

pub fn get_case_sensitive_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CASE_SENSITIVE_V1)
}

pub fn get_sentence_terminal_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_TERMINAL_V1)
}

pub fn get_terminal_punctuation_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TERMINAL_PUNCTUATION_V1)
}

pub fn get_unified_ideograph_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::UNIFIED_IDEOGRAPH_V1)
}

pub fn get_uppercase_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::UPPERCASE_V1)
}

pub fn get_variation_selector_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::VARIATION_SELECTOR_V1)
}

pub fn get_white_space_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WHITE_SPACE_V1)
}

pub fn get_xdigit_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::XDIGIT_V1)
}

pub fn get_xid_continue_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::XID_CONTINUE_V1)
}

pub fn get_xid_start_property<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::XID_START_V1)
}

//
// Enumerated property getter fns
//

pub fn get_bidi_class_arabic_letter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_ARABIC_LETTER_V1)
}

pub fn get_bidi_class_arabic_number<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_ARABIC_NUMBER_V1)
}

pub fn get_bidi_class_paragraph_separator<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_PARAGRAPH_SEPARATOR_V1)
}

pub fn get_bidi_class_boundary_neutral<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_BOUNDARY_NEUTRAL_V1)
}

pub fn get_bidi_class_common_separator<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_COMMON_SEPARATOR_V1)
}

pub fn get_bidi_class_european_number<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_EUROPEAN_NUMBER_V1)
}

pub fn get_bidi_class_european_separator<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_EUROPEAN_SEPARATOR_V1)
}

pub fn get_bidi_class_european_terminator<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_EUROPEAN_TERMINATOR_V1)
}

pub fn get_bidi_class_first_strong_isolate<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_FIRST_STRONG_ISOLATE_V1)
}

pub fn get_bidi_class_left_to_right<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_LEFT_TO_RIGHT_V1)
}

pub fn get_bidi_class_left_to_right_embedding<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_LEFT_TO_RIGHT_EMBEDDING_V1)
}

pub fn get_bidi_class_left_to_right_isolate<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_LEFT_TO_RIGHT_ISOLATE_V1)
}

pub fn get_bidi_class_left_to_right_override<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_LEFT_TO_RIGHT_OVERRIDE_V1)
}

pub fn get_bidi_class_nonspacing_mark<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_NONSPACING_MARK_V1)
}

pub fn get_bidi_class_other_neutral<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_OTHER_NEUTRAL_V1)
}

pub fn get_bidi_class_pop_directional_format<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_POP_DIRECTIONAL_FORMAT_V1)
}

pub fn get_bidi_class_pop_directional_isolate<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_POP_DIRECTIONAL_ISOLATE_V1)
}

pub fn get_bidi_class_right_to_left<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_RIGHT_TO_LEFT_V1)
}

pub fn get_bidi_class_right_to_left_embedding<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_RIGHT_TO_LEFT_EMBEDDING_V1)
}

pub fn get_bidi_class_right_to_left_isolate<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_RIGHT_TO_LEFT_ISOLATE_V1)
}

pub fn get_bidi_class_right_to_left_override<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_RIGHT_TO_LEFT_OVERRIDE_V1)
}

pub fn get_bidi_class_segment_separator<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_SEGMENT_SEPARATOR_V1)
}

pub fn get_bidi_class_white_space<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_CLASS_WHITE_SPACE_V1)
}

pub fn get_bidi_paired_bracket_type_close<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_PAIRED_BRACKET_TYPE_CLOSE_V1)
}

pub fn get_bidi_paired_bracket_type_none<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_PAIRED_BRACKET_TYPE_NONE_V1)
}

pub fn get_bidi_paired_bracket_type_open<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::BIDI_PAIRED_BRACKET_TYPE_OPEN_V1)
}

pub fn get_canonical_combining_class_not_reordered<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1)
}

pub fn get_canonical_combining_class_overlay<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_OVERLAY_V1)
}

pub fn get_canonical_combining_class_ccc10<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC10_V1)
}

pub fn get_canonical_combining_class_ccc103<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC103_V1)
}

pub fn get_canonical_combining_class_ccc107<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC107_V1)
}

pub fn get_canonical_combining_class_ccc11<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC11_V1)
}

pub fn get_canonical_combining_class_ccc118<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC118_V1)
}

pub fn get_canonical_combining_class_ccc12<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC12_V1)
}

pub fn get_canonical_combining_class_ccc122<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC122_V1)
}

pub fn get_canonical_combining_class_ccc129<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC129_V1)
}

pub fn get_canonical_combining_class_ccc13<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC13_V1)
}

pub fn get_canonical_combining_class_ccc130<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC130_V1)
}

pub fn get_canonical_combining_class_ccc132<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC132_V1)
}

pub fn get_canonical_combining_class_ccc133<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC133_V1)
}

pub fn get_canonical_combining_class_ccc14<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC14_V1)
}

pub fn get_canonical_combining_class_ccc15<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC15_V1)
}

pub fn get_canonical_combining_class_ccc16<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC16_V1)
}

pub fn get_canonical_combining_class_ccc17<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC17_V1)
}

pub fn get_canonical_combining_class_ccc18<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC18_V1)
}

pub fn get_canonical_combining_class_ccc19<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC19_V1)
}

pub fn get_canonical_combining_class_ccc20<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC20_V1)
}

pub fn get_canonical_combining_class_attached_below_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1,
    )
}

pub fn get_canonical_combining_class_attached_below<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1)
}

pub fn get_canonical_combining_class_ccc21<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC21_V1)
}

pub fn get_canonical_combining_class_attached_above<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1)
}

pub fn get_canonical_combining_class_attached_above_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1,
    )
}

pub fn get_canonical_combining_class_below_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1)
}

pub fn get_canonical_combining_class_ccc22<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC22_V1)
}

pub fn get_canonical_combining_class_below<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_BELOW_V1)
}

pub fn get_canonical_combining_class_below_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1)
}

pub fn get_canonical_combining_class_left<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_LEFT_V1)
}

pub fn get_canonical_combining_class_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_RIGHT_V1)
}

pub fn get_canonical_combining_class_above_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1)
}

pub fn get_canonical_combining_class_ccc23<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC23_V1)
}

pub fn get_canonical_combining_class_above<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_ABOVE_V1)
}

pub fn get_canonical_combining_class_above_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1)
}

pub fn get_canonical_combining_class_double_below<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1)
}

pub fn get_canonical_combining_class_double_above<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1)
}

pub fn get_canonical_combining_class_ccc24<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC24_V1)
}

pub fn get_canonical_combining_class_iota_subscript<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1)
}

pub fn get_canonical_combining_class_ccc25<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC25_V1)
}

pub fn get_canonical_combining_class_ccc26<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC26_V1)
}

pub fn get_canonical_combining_class_ccc27<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC27_V1)
}

pub fn get_canonical_combining_class_ccc28<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC28_V1)
}

pub fn get_canonical_combining_class_ccc29<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC29_V1)
}

pub fn get_canonical_combining_class_ccc30<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC30_V1)
}

pub fn get_canonical_combining_class_ccc31<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC31_V1)
}

pub fn get_canonical_combining_class_ccc32<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC32_V1)
}

pub fn get_canonical_combining_class_ccc33<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC33_V1)
}

pub fn get_canonical_combining_class_ccc34<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC34_V1)
}

pub fn get_canonical_combining_class_ccc35<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC35_V1)
}

pub fn get_canonical_combining_class_ccc36<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC36_V1)
}

pub fn get_canonical_combining_class_han_reading<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_HAN_READING_V1)
}

pub fn get_canonical_combining_class_nukta<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_NUKTA_V1)
}

pub fn get_canonical_combining_class_kana_voicing<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_KANA_VOICING_V1)
}

pub fn get_canonical_combining_class_ccc84<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC84_V1)
}

pub fn get_canonical_combining_class_virama<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_VIRAMA_V1)
}

pub fn get_canonical_combining_class_ccc91<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::CANONICAL_COMBINING_CLASS_CCC91_V1)
}

pub fn get_decomposition_type_can<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_CAN_V1)
}

pub fn get_decomposition_type_com<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_COM_V1)
}

pub fn get_decomposition_type_enc<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_ENC_V1)
}

pub fn get_decomposition_type_fin<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_FIN_V1)
}

pub fn get_decomposition_type_font<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_FONT_V1)
}

pub fn get_decomposition_type_fra<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_FRA_V1)
}

pub fn get_decomposition_type_init<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_INIT_V1)
}

pub fn get_decomposition_type_iso<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_ISO_V1)
}

pub fn get_decomposition_type_med<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_MED_V1)
}

pub fn get_decomposition_type_nar<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_NAR_V1)
}

pub fn get_decomposition_type_nb<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_NB_V1)
}

pub fn get_decomposition_type_none<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_NONE_V1)
}

pub fn get_decomposition_type_sml<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_SML_V1)
}

pub fn get_decomposition_type_sqr<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_SQR_V1)
}

pub fn get_decomposition_type_sub<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_SUB_V1)
}

pub fn get_decomposition_type_sup<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_SUP_V1)
}

pub fn get_decomposition_type_vert<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_VERT_V1)
}

pub fn get_decomposition_type_wide<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::DECOMPOSITION_TYPE_WIDE_V1)
}

pub fn get_east_asian_width_ambiguous<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EAST_ASIAN_WIDTH_AMBIGUOUS_V1)
}

pub fn get_east_asian_width_fullwidth<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EAST_ASIAN_WIDTH_FULLWIDTH_V1)
}

pub fn get_east_asian_width_halfwidth<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EAST_ASIAN_WIDTH_HALFWIDTH_V1)
}

pub fn get_east_asian_width_neutral<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EAST_ASIAN_WIDTH_NEUTRAL_V1)
}

pub fn get_east_asian_width_narrow<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EAST_ASIAN_WIDTH_NARROW_V1)
}

pub fn get_east_asian_width_wide<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::EAST_ASIAN_WIDTH_WIDE_V1)
}

pub fn get_general_category_other<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_OTHER_V1)
}

pub fn get_general_category_cntrl<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_CNTRL_V1)
}

pub fn get_general_category_format<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_FORMAT_V1)
}

pub fn get_general_category_unassigned<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_UNASSIGNED_V1)
}

pub fn get_general_category_private_use<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_PRIVATE_USE_V1)
}

pub fn get_general_category_surrogate<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_SURROGATE_V1)
}

pub fn get_general_category_letter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_LETTER_V1)
}

pub fn get_general_category_cased_letter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_CASED_LETTER_V1)
}

pub fn get_general_category_lowercase_letter<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_LOWERCASE_LETTER_V1)
}

pub fn get_general_category_modifier_letter<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_MODIFIER_LETTER_V1)
}

pub fn get_general_category_other_letter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_OTHER_LETTER_V1)
}

pub fn get_general_category_titlecase_letter<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_TITLECASE_LETTER_V1)
}

pub fn get_general_category_uppercase_letter<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_UPPERCASE_LETTER_V1)
}

pub fn get_general_category_combining_mark<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_COMBINING_MARK_V1)
}

pub fn get_general_category_spacing_mark<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_SPACING_MARK_V1)
}

pub fn get_general_category_enclosing_mark<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_ENCLOSING_MARK_V1)
}

pub fn get_general_category_nonspacing_mark<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_NONSPACING_MARK_V1)
}

pub fn get_general_category_number<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_NUMBER_V1)
}

pub fn get_general_category_digit<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_DIGIT_V1)
}

pub fn get_general_category_letter_number<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_LETTER_NUMBER_V1)
}

pub fn get_general_category_other_number<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_OTHER_NUMBER_V1)
}

pub fn get_general_category_punct<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_PUNCT_V1)
}

pub fn get_general_category_connector_punctuation<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_CONNECTOR_PUNCTUATION_V1)
}

pub fn get_general_category_dash_punctuation<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_DASH_PUNCTUATION_V1)
}

pub fn get_general_category_close_punctuation<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_CLOSE_PUNCTUATION_V1)
}

pub fn get_general_category_final_punctuation<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_FINAL_PUNCTUATION_V1)
}

pub fn get_general_category_initial_punctuation<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_INITIAL_PUNCTUATION_V1)
}

pub fn get_general_category_other_punctuation<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_OTHER_PUNCTUATION_V1)
}

pub fn get_general_category_open_punctuation<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_OPEN_PUNCTUATION_V1)
}

pub fn get_general_category_symbol<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_SYMBOL_V1)
}

pub fn get_general_category_currency_symbol<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_CURRENCY_SYMBOL_V1)
}

pub fn get_general_category_modifier_symbol<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_MODIFIER_SYMBOL_V1)
}

pub fn get_general_category_math_symbol<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_MATH_SYMBOL_V1)
}

pub fn get_general_category_other_symbol<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_OTHER_SYMBOL_V1)
}

pub fn get_general_category_separator<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_SEPARATOR_V1)
}

pub fn get_general_category_line_separator<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_LINE_SEPARATOR_V1)
}

pub fn get_general_category_paragraph_separator<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_PARAGRAPH_SEPARATOR_V1)
}

pub fn get_general_category_space_separator<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GENERAL_CATEGORY_SPACE_SEPARATOR_V1)
}

pub fn get_grapheme_cluster_break_control<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_CONTROL_V1)
}

pub fn get_grapheme_cluster_break_cr<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_CR_V1)
}

pub fn get_grapheme_cluster_break_e_base<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_E_BASE_V1)
}

pub fn get_grapheme_cluster_break_e_base_gaz<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_E_BASE_GAZ_V1)
}

pub fn get_grapheme_cluster_break_e_modifier<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_E_MODIFIER_V1)
}

pub fn get_grapheme_cluster_break_extend<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_EXTEND_V1)
}

pub fn get_grapheme_cluster_break_glue_after_zwj<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_GLUE_AFTER_ZWJ_V1)
}

pub fn get_grapheme_cluster_break_l<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_L_V1)
}

pub fn get_grapheme_cluster_break_lf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_LF_V1)
}

pub fn get_grapheme_cluster_break_lv<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_LV_V1)
}

pub fn get_grapheme_cluster_break_lvt<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_LVT_V1)
}

pub fn get_grapheme_cluster_break_prepend<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_PREPEND_V1)
}

pub fn get_grapheme_cluster_break_regional_indicator<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_REGIONAL_INDICATOR_V1)
}

pub fn get_grapheme_cluster_break_spacingmark<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_SPACINGMARK_V1)
}

pub fn get_grapheme_cluster_break_t<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_T_V1)
}

pub fn get_grapheme_cluster_break_v<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_V_V1)
}

pub fn get_grapheme_cluster_break_other<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_OTHER_V1)
}

pub fn get_grapheme_cluster_break_zwj<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::GRAPHEME_CLUSTER_BREAK_ZWJ_V1)
}

pub fn get_hangul_syllable_type_leading_jamo<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HANGUL_SYLLABLE_TYPE_LEADING_JAMO_V1)
}

pub fn get_hangul_syllable_type_lv_syllable<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HANGUL_SYLLABLE_TYPE_LV_SYLLABLE_V1)
}

pub fn get_hangul_syllable_type_lvt_syllable<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HANGUL_SYLLABLE_TYPE_LVT_SYLLABLE_V1)
}

pub fn get_hangul_syllable_type_not_applicable<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HANGUL_SYLLABLE_TYPE_NOT_APPLICABLE_V1)
}

pub fn get_hangul_syllable_type_trailing_jamo<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HANGUL_SYLLABLE_TYPE_TRAILING_JAMO_V1)
}

pub fn get_hangul_syllable_type_vowel_jamo<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::HANGUL_SYLLABLE_TYPE_VOWEL_JAMO_V1)
}

pub fn get_indic_positional_category_bottom<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_BOTTOM_V1)
}

pub fn get_indic_positional_category_bottom_and_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_BOTTOM_AND_LEFT_V1)
}

pub fn get_indic_positional_category_bottom_and_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_BOTTOM_AND_RIGHT_V1)
}

pub fn get_indic_positional_category_left<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_LEFT_V1)
}

pub fn get_indic_positional_category_left_and_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_LEFT_AND_RIGHT_V1)
}

pub fn get_indic_positional_category_na<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_NA_V1)
}

pub fn get_indic_positional_category_overstruck<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_OVERSTRUCK_V1)
}

pub fn get_indic_positional_category_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_RIGHT_V1)
}

pub fn get_indic_positional_category_top<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_V1)
}

pub fn get_indic_positional_category_top_and_bottom<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_V1)
}

pub fn get_indic_positional_category_top_and_bottom_and_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_AND_LEFT_V1,
    )
}

pub fn get_indic_positional_category_top_and_bottom_and_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_POSITIONAL_CATEGORY_TOP_AND_BOTTOM_AND_RIGHT_V1,
    )
}

pub fn get_indic_positional_category_top_and_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_AND_LEFT_V1)
}

pub fn get_indic_positional_category_top_and_left_and_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_POSITIONAL_CATEGORY_TOP_AND_LEFT_AND_RIGHT_V1,
    )
}

pub fn get_indic_positional_category_top_and_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_POSITIONAL_CATEGORY_TOP_AND_RIGHT_V1)
}

pub fn get_indic_positional_category_visual_order_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_POSITIONAL_CATEGORY_VISUAL_ORDER_LEFT_V1,
    )
}

pub fn get_indic_syllabic_category_avagraha<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_AVAGRAHA_V1)
}

pub fn get_indic_syllabic_category_bindu<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_BINDU_V1)
}

pub fn get_indic_syllabic_category_brahmi_joining_number<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_SYLLABIC_CATEGORY_BRAHMI_JOINING_NUMBER_V1,
    )
}

pub fn get_indic_syllabic_category_cantillation_mark<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CANTILLATION_MARK_V1)
}

pub fn get_indic_syllabic_category_consonant<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_V1)
}

pub fn get_indic_syllabic_category_consonant_dead<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_DEAD_V1)
}

pub fn get_indic_syllabic_category_consonant_final<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_FINAL_V1)
}

pub fn get_indic_syllabic_category_consonant_head_letter<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_SYLLABIC_CATEGORY_CONSONANT_HEAD_LETTER_V1,
    )
}

pub fn get_indic_syllabic_category_consonant_initial_postfixed<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_SYLLABIC_CATEGORY_CONSONANT_INITIAL_POSTFIXED_V1,
    )
}

pub fn get_indic_syllabic_category_consonant_killer<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_KILLER_V1)
}

pub fn get_indic_syllabic_category_consonant_medial<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_MEDIAL_V1)
}

pub fn get_indic_syllabic_category_consonant_placeholder<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_SYLLABIC_CATEGORY_CONSONANT_PLACEHOLDER_V1,
    )
}

pub fn get_indic_syllabic_category_consonant_preceding_repha<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_SYLLABIC_CATEGORY_CONSONANT_PRECEDING_REPHA_V1,
    )
}

pub fn get_indic_syllabic_category_consonant_prefixed<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_CONSONANT_PREFIXED_V1)
}

pub fn get_indic_syllabic_category_consonant_subjoined<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_SYLLABIC_CATEGORY_CONSONANT_SUBJOINED_V1,
    )
}

pub fn get_indic_syllabic_category_consonant_succeeding_repha<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_SYLLABIC_CATEGORY_CONSONANT_SUCCEEDING_REPHA_V1,
    )
}

pub fn get_indic_syllabic_category_consonant_with_stacker<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::INDIC_SYLLABIC_CATEGORY_CONSONANT_WITH_STACKER_V1,
    )
}

pub fn get_indic_syllabic_category_gemination_mark<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_GEMINATION_MARK_V1)
}

pub fn get_indic_syllabic_category_invisible_stacker<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_INVISIBLE_STACKER_V1)
}

pub fn get_indic_syllabic_category_joiner<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_JOINER_V1)
}

pub fn get_indic_syllabic_category_modifying_letter<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_MODIFYING_LETTER_V1)
}

pub fn get_indic_syllabic_category_non_joiner<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_NON_JOINER_V1)
}

pub fn get_indic_syllabic_category_nukta<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_NUKTA_V1)
}

pub fn get_indic_syllabic_category_number<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_NUMBER_V1)
}

pub fn get_indic_syllabic_category_number_joiner<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_NUMBER_JOINER_V1)
}

pub fn get_indic_syllabic_category_other<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_OTHER_V1)
}

pub fn get_indic_syllabic_category_pure_killer<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_PURE_KILLER_V1)
}

pub fn get_indic_syllabic_category_register_shifter<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_REGISTER_SHIFTER_V1)
}

pub fn get_indic_syllabic_category_syllable_modifier<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_SYLLABLE_MODIFIER_V1)
}

pub fn get_indic_syllabic_category_tone_letter<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_TONE_LETTER_V1)
}

pub fn get_indic_syllabic_category_tone_mark<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_TONE_MARK_V1)
}

pub fn get_indic_syllabic_category_virama<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VIRAMA_V1)
}

pub fn get_indic_syllabic_category_visarga<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VISARGA_V1)
}

pub fn get_indic_syllabic_category_vowel<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VOWEL_V1)
}

pub fn get_indic_syllabic_category_vowel_dependent<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VOWEL_DEPENDENT_V1)
}

pub fn get_indic_syllabic_category_vowel_independent<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::INDIC_SYLLABIC_CATEGORY_VOWEL_INDEPENDENT_V1)
}

pub fn get_joining_group_african_feh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_AFRICAN_FEH_V1)
}

pub fn get_joining_group_african_noon<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_AFRICAN_NOON_V1)
}

pub fn get_joining_group_african_qaf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_AFRICAN_QAF_V1)
}

pub fn get_joining_group_ain<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_AIN_V1)
}

pub fn get_joining_group_alaph<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_ALAPH_V1)
}

pub fn get_joining_group_alef<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_ALEF_V1)
}

pub fn get_joining_group_beh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_BEH_V1)
}

pub fn get_joining_group_beth<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_BETH_V1)
}

pub fn get_joining_group_burushaski_yeh_barree<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_BURUSHASKI_YEH_BARREE_V1)
}

pub fn get_joining_group_dal<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_DAL_V1)
}

pub fn get_joining_group_dalath_rish<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_DALATH_RISH_V1)
}

pub fn get_joining_group_e<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_E_V1)
}

pub fn get_joining_group_farsi_yeh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_FARSI_YEH_V1)
}

pub fn get_joining_group_fe<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_FE_V1)
}

pub fn get_joining_group_feh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_FEH_V1)
}

pub fn get_joining_group_final_semkath<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_FINAL_SEMKATH_V1)
}

pub fn get_joining_group_gaf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_GAF_V1)
}

pub fn get_joining_group_gamal<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_GAMAL_V1)
}

pub fn get_joining_group_hah<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_HAH_V1)
}

pub fn get_joining_group_hanifi_rohingya_kinna_ya<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_HANIFI_ROHINGYA_KINNA_YA_V1)
}

pub fn get_joining_group_hanifi_rohingya_pa<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_HANIFI_ROHINGYA_PA_V1)
}

pub fn get_joining_group_he<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_HE_V1)
}

pub fn get_joining_group_heh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_HEH_V1)
}

pub fn get_joining_group_heh_goal<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_HEH_GOAL_V1)
}

pub fn get_joining_group_heth<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_HETH_V1)
}

pub fn get_joining_group_kaf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_KAF_V1)
}

pub fn get_joining_group_kaph<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_KAPH_V1)
}

pub fn get_joining_group_khaph<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_KHAPH_V1)
}

pub fn get_joining_group_knotted_heh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_KNOTTED_HEH_V1)
}

pub fn get_joining_group_lam<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_LAM_V1)
}

pub fn get_joining_group_lamadh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_LAMADH_V1)
}

pub fn get_joining_group_malayalam_bha<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_BHA_V1)
}

pub fn get_joining_group_malayalam_ja<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_JA_V1)
}

pub fn get_joining_group_malayalam_lla<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_LLA_V1)
}

pub fn get_joining_group_malayalam_llla<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_LLLA_V1)
}

pub fn get_joining_group_malayalam_nga<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_NGA_V1)
}

pub fn get_joining_group_malayalam_nna<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_NNA_V1)
}

pub fn get_joining_group_malayalam_nnna<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_NNNA_V1)
}

pub fn get_joining_group_malayalam_nya<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_NYA_V1)
}

pub fn get_joining_group_malayalam_ra<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_RA_V1)
}

pub fn get_joining_group_malayalam_ssa<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_SSA_V1)
}

pub fn get_joining_group_malayalam_tta<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MALAYALAM_TTA_V1)
}

pub fn get_joining_group_manichaean_aleph<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_ALEPH_V1)
}

pub fn get_joining_group_manichaean_ayin<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_AYIN_V1)
}

pub fn get_joining_group_manichaean_beth<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_BETH_V1)
}

pub fn get_joining_group_manichaean_daleth<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_DALETH_V1)
}

pub fn get_joining_group_manichaean_dhamedh<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_DHAMEDH_V1)
}

pub fn get_joining_group_manichaean_five<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_FIVE_V1)
}

pub fn get_joining_group_manichaean_gimel<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_GIMEL_V1)
}

pub fn get_joining_group_manichaean_heth<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_HETH_V1)
}

pub fn get_joining_group_manichaean_hundred<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_HUNDRED_V1)
}

pub fn get_joining_group_manichaean_kaph<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_KAPH_V1)
}

pub fn get_joining_group_manichaean_lamedh<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_LAMEDH_V1)
}

pub fn get_joining_group_manichaean_mem<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_MEM_V1)
}

pub fn get_joining_group_manichaean_nun<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_NUN_V1)
}

pub fn get_joining_group_manichaean_one<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_ONE_V1)
}

pub fn get_joining_group_manichaean_pe<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_PE_V1)
}

pub fn get_joining_group_manichaean_qoph<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_QOPH_V1)
}

pub fn get_joining_group_manichaean_resh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_RESH_V1)
}

pub fn get_joining_group_manichaean_sadhe<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_SADHE_V1)
}

pub fn get_joining_group_manichaean_samekh<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_SAMEKH_V1)
}

pub fn get_joining_group_manichaean_taw<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_TAW_V1)
}

pub fn get_joining_group_manichaean_ten<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_TEN_V1)
}

pub fn get_joining_group_manichaean_teth<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_TETH_V1)
}

pub fn get_joining_group_manichaean_thamedh<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_THAMEDH_V1)
}

pub fn get_joining_group_manichaean_twenty<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_TWENTY_V1)
}

pub fn get_joining_group_manichaean_waw<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_WAW_V1)
}

pub fn get_joining_group_manichaean_yodh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_YODH_V1)
}

pub fn get_joining_group_manichaean_zayin<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MANICHAEAN_ZAYIN_V1)
}

pub fn get_joining_group_meem<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MEEM_V1)
}

pub fn get_joining_group_mim<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_MIM_V1)
}

pub fn get_joining_group_no_joining_group<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_NO_JOINING_GROUP_V1)
}

pub fn get_joining_group_noon<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_NOON_V1)
}

pub fn get_joining_group_nun<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_NUN_V1)
}

pub fn get_joining_group_nya<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_NYA_V1)
}

pub fn get_joining_group_pe<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_PE_V1)
}

pub fn get_joining_group_qaf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_QAF_V1)
}

pub fn get_joining_group_qaph<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_QAPH_V1)
}

pub fn get_joining_group_reh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_REH_V1)
}

pub fn get_joining_group_reversed_pe<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_REVERSED_PE_V1)
}

pub fn get_joining_group_rohingya_yeh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_ROHINGYA_YEH_V1)
}

pub fn get_joining_group_sad<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_SAD_V1)
}

pub fn get_joining_group_sadhe<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_SADHE_V1)
}

pub fn get_joining_group_seen<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_SEEN_V1)
}

pub fn get_joining_group_semkath<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_SEMKATH_V1)
}

pub fn get_joining_group_shin<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_SHIN_V1)
}

pub fn get_joining_group_straight_waw<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_STRAIGHT_WAW_V1)
}

pub fn get_joining_group_swash_kaf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_SWASH_KAF_V1)
}

pub fn get_joining_group_syriac_waw<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_SYRIAC_WAW_V1)
}

pub fn get_joining_group_tah<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_TAH_V1)
}

pub fn get_joining_group_taw<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_TAW_V1)
}

pub fn get_joining_group_teh_marbuta<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_TEH_MARBUTA_V1)
}

pub fn get_joining_group_hamza_on_heh_goal<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_HAMZA_ON_HEH_GOAL_V1)
}

pub fn get_joining_group_teth<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_TETH_V1)
}

pub fn get_joining_group_waw<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_WAW_V1)
}

pub fn get_joining_group_yeh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_YEH_V1)
}

pub fn get_joining_group_yeh_barree<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_YEH_BARREE_V1)
}

pub fn get_joining_group_yeh_with_tail<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_YEH_WITH_TAIL_V1)
}

pub fn get_joining_group_yudh<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_YUDH_V1)
}

pub fn get_joining_group_yudh_he<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_YUDH_HE_V1)
}

pub fn get_joining_group_zain<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_ZAIN_V1)
}

pub fn get_joining_group_zhain<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_GROUP_ZHAIN_V1)
}

pub fn get_joining_type_join_causing<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_TYPE_JOIN_CAUSING_V1)
}

pub fn get_joining_type_dual_joining<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_TYPE_DUAL_JOINING_V1)
}

pub fn get_joining_type_left_joining<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_TYPE_LEFT_JOINING_V1)
}

pub fn get_joining_type_right_joining<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_TYPE_RIGHT_JOINING_V1)
}

pub fn get_joining_type_transparent<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_TYPE_TRANSPARENT_V1)
}

pub fn get_joining_type_non_joining<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::JOINING_TYPE_NON_JOINING_V1)
}

pub fn get_line_break_ambiguous<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_AMBIGUOUS_V1)
}

pub fn get_line_break_alphabetic<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_ALPHABETIC_V1)
}

pub fn get_line_break_break_both<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_BREAK_BOTH_V1)
}

pub fn get_line_break_break_after<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_BREAK_AFTER_V1)
}

pub fn get_line_break_break_before<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_BREAK_BEFORE_V1)
}

pub fn get_line_break_mandatory_break<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_MANDATORY_BREAK_V1)
}

pub fn get_line_break_contingent_break<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_CONTINGENT_BREAK_V1)
}

pub fn get_line_break_conditional_japanese_starter<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_CONDITIONAL_JAPANESE_STARTER_V1)
}

pub fn get_line_break_close_punctuation<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_CLOSE_PUNCTUATION_V1)
}

pub fn get_line_break_combining_mark<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_COMBINING_MARK_V1)
}

pub fn get_line_break_close_parenthesis<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_CLOSE_PARENTHESIS_V1)
}

pub fn get_line_break_carriage_return<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_CARRIAGE_RETURN_V1)
}

pub fn get_line_break_e_base<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_E_BASE_V1)
}

pub fn get_line_break_e_modifier<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_E_MODIFIER_V1)
}

pub fn get_line_break_exclamation<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_EXCLAMATION_V1)
}

pub fn get_line_break_glue<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_GLUE_V1)
}

pub fn get_line_break_h2<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_H2_V1)
}

pub fn get_line_break_h3<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_H3_V1)
}

pub fn get_line_break_hebrew_letter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_HEBREW_LETTER_V1)
}

pub fn get_line_break_hyphen<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_HYPHEN_V1)
}

pub fn get_line_break_ideographic<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_IDEOGRAPHIC_V1)
}

pub fn get_line_break_inseperable<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_INSEPERABLE_V1)
}

pub fn get_line_break_infix_numeric<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_INFIX_NUMERIC_V1)
}

pub fn get_line_break_jl<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_JL_V1)
}

pub fn get_line_break_jt<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_JT_V1)
}

pub fn get_line_break_jv<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_JV_V1)
}

pub fn get_line_break_line_feed<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_LINE_FEED_V1)
}

pub fn get_line_break_next_line<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_NEXT_LINE_V1)
}

pub fn get_line_break_nonstarter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_NONSTARTER_V1)
}

pub fn get_line_break_numeric<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_NUMERIC_V1)
}

pub fn get_line_break_open_punctuation<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_OPEN_PUNCTUATION_V1)
}

pub fn get_line_break_postfix_numeric<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_POSTFIX_NUMERIC_V1)
}

pub fn get_line_break_prefix_numeric<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_PREFIX_NUMERIC_V1)
}

pub fn get_line_break_quotation<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_QUOTATION_V1)
}

pub fn get_line_break_regional_indicator<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_REGIONAL_INDICATOR_V1)
}

pub fn get_line_break_complex_context<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_COMPLEX_CONTEXT_V1)
}

pub fn get_line_break_surrogate<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_SURROGATE_V1)
}

pub fn get_line_break_space<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_SPACE_V1)
}

pub fn get_line_break_break_symbols<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_BREAK_SYMBOLS_V1)
}

pub fn get_line_break_word_joiner<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_WORD_JOINER_V1)
}

pub fn get_line_break_unknown<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_UNKNOWN_V1)
}

pub fn get_line_break_zwspace<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_ZWSPACE_V1)
}

pub fn get_line_break_zwj<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LINE_BREAK_ZWJ_V1)
}

pub fn get_lead_canonical_combining_class_not_reordered<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::LEAD_CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1,
    )
}

pub fn get_lead_canonical_combining_class_overlay<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_OVERLAY_V1)
}

pub fn get_lead_canonical_combining_class_ccc10<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC10_V1)
}

pub fn get_lead_canonical_combining_class_ccc103<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC103_V1)
}

pub fn get_lead_canonical_combining_class_ccc107<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC107_V1)
}

pub fn get_lead_canonical_combining_class_ccc11<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC11_V1)
}

pub fn get_lead_canonical_combining_class_ccc118<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC118_V1)
}

pub fn get_lead_canonical_combining_class_ccc12<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC12_V1)
}

pub fn get_lead_canonical_combining_class_ccc122<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC122_V1)
}

pub fn get_lead_canonical_combining_class_ccc129<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC129_V1)
}

pub fn get_lead_canonical_combining_class_ccc13<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC13_V1)
}

pub fn get_lead_canonical_combining_class_ccc130<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC130_V1)
}

pub fn get_lead_canonical_combining_class_ccc132<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC132_V1)
}

pub fn get_lead_canonical_combining_class_ccc133<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC133_V1)
}

pub fn get_lead_canonical_combining_class_ccc14<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC14_V1)
}

pub fn get_lead_canonical_combining_class_ccc15<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC15_V1)
}

pub fn get_lead_canonical_combining_class_ccc16<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC16_V1)
}

pub fn get_lead_canonical_combining_class_ccc17<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC17_V1)
}

pub fn get_lead_canonical_combining_class_ccc18<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC18_V1)
}

pub fn get_lead_canonical_combining_class_ccc19<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC19_V1)
}

pub fn get_lead_canonical_combining_class_ccc20<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC20_V1)
}

pub fn get_lead_canonical_combining_class_attached_below_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1,
    )
}

pub fn get_lead_canonical_combining_class_attached_below<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1,
    )
}

pub fn get_lead_canonical_combining_class_ccc21<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC21_V1)
}

pub fn get_lead_canonical_combining_class_attached_above<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1,
    )
}

pub fn get_lead_canonical_combining_class_attached_above_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::LEAD_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1,
    )
}

pub fn get_lead_canonical_combining_class_below_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1)
}

pub fn get_lead_canonical_combining_class_ccc22<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC22_V1)
}

pub fn get_lead_canonical_combining_class_below<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_BELOW_V1)
}

pub fn get_lead_canonical_combining_class_below_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1)
}

pub fn get_lead_canonical_combining_class_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_LEFT_V1)
}

pub fn get_lead_canonical_combining_class_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_RIGHT_V1)
}

pub fn get_lead_canonical_combining_class_above_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1)
}

pub fn get_lead_canonical_combining_class_ccc23<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC23_V1)
}

pub fn get_lead_canonical_combining_class_above<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ABOVE_V1)
}

pub fn get_lead_canonical_combining_class_above_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1)
}

pub fn get_lead_canonical_combining_class_double_below<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::LEAD_CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1,
    )
}

pub fn get_lead_canonical_combining_class_double_above<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::LEAD_CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1,
    )
}

pub fn get_lead_canonical_combining_class_ccc24<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC24_V1)
}

pub fn get_lead_canonical_combining_class_iota_subscript<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::LEAD_CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1,
    )
}

pub fn get_lead_canonical_combining_class_ccc25<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC25_V1)
}

pub fn get_lead_canonical_combining_class_ccc26<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC26_V1)
}

pub fn get_lead_canonical_combining_class_ccc27<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC27_V1)
}

pub fn get_lead_canonical_combining_class_ccc28<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC28_V1)
}

pub fn get_lead_canonical_combining_class_ccc29<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC29_V1)
}

pub fn get_lead_canonical_combining_class_ccc30<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC30_V1)
}

pub fn get_lead_canonical_combining_class_ccc31<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC31_V1)
}

pub fn get_lead_canonical_combining_class_ccc32<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC32_V1)
}

pub fn get_lead_canonical_combining_class_ccc33<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC33_V1)
}

pub fn get_lead_canonical_combining_class_ccc34<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC34_V1)
}

pub fn get_lead_canonical_combining_class_ccc35<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC35_V1)
}

pub fn get_lead_canonical_combining_class_ccc36<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC36_V1)
}

pub fn get_lead_canonical_combining_class_han_reading<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_HAN_READING_V1)
}

pub fn get_lead_canonical_combining_class_nukta<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_NUKTA_V1)
}

pub fn get_lead_canonical_combining_class_kana_voicing<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::LEAD_CANONICAL_COMBINING_CLASS_KANA_VOICING_V1,
    )
}

pub fn get_lead_canonical_combining_class_ccc84<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC84_V1)
}

pub fn get_lead_canonical_combining_class_virama<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_VIRAMA_V1)
}

pub fn get_lead_canonical_combining_class_ccc91<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::LEAD_CANONICAL_COMBINING_CLASS_CCC91_V1)
}

pub fn get_nfc_quick_check_maybe<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFC_QUICK_CHECK_MAYBE_V1)
}

pub fn get_nfc_quick_check_no<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFC_QUICK_CHECK_NO_V1)
}

pub fn get_nfc_quick_check_yes<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFC_QUICK_CHECK_YES_V1)
}

pub fn get_nfd_quick_check_no<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFD_QUICK_CHECK_NO_V1)
}

pub fn get_nfd_quick_check_yes<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFD_QUICK_CHECK_YES_V1)
}

pub fn get_nfkc_quick_check_maybe<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFKC_QUICK_CHECK_MAYBE_V1)
}

pub fn get_nfkc_quick_check_no<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFKC_QUICK_CHECK_NO_V1)
}

pub fn get_nfkc_quick_check_yes<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFKC_QUICK_CHECK_YES_V1)
}

pub fn get_nfkd_quick_check_no<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFKD_QUICK_CHECK_NO_V1)
}

pub fn get_nfkd_quick_check_yes<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NFKD_QUICK_CHECK_YES_V1)
}

pub fn get_numeric_type_decimal<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NUMERIC_TYPE_DECIMAL_V1)
}

pub fn get_numeric_type_digit<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NUMERIC_TYPE_DIGIT_V1)
}

pub fn get_numeric_type_none<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NUMERIC_TYPE_NONE_V1)
}

pub fn get_numeric_type_numeric<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::NUMERIC_TYPE_NUMERIC_V1)
}

pub fn get_sentence_break_aterm<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_ATERM_V1)
}

pub fn get_sentence_break_close<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_CLOSE_V1)
}

pub fn get_sentence_break_cr<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_CR_V1)
}

pub fn get_sentence_break_extend<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_EXTEND_V1)
}

pub fn get_sentence_break_format<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_FORMAT_V1)
}

pub fn get_sentence_break_oletter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_OLETTER_V1)
}

pub fn get_sentence_break_lf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_LF_V1)
}

pub fn get_sentence_break_lower<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_LOWER_V1)
}

pub fn get_sentence_break_numeric<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_NUMERIC_V1)
}

pub fn get_sentence_break_scontinue<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_SCONTINUE_V1)
}

pub fn get_sentence_break_sep<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_SEP_V1)
}

pub fn get_sentence_break_sp<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_SP_V1)
}

pub fn get_sentence_break_sterm<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_STERM_V1)
}

pub fn get_sentence_break_upper<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_UPPER_V1)
}

pub fn get_sentence_break_other<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::SENTENCE_BREAK_OTHER_V1)
}

pub fn get_trail_canonical_combining_class_not_reordered<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_NOT_REORDERED_V1,
    )
}

pub fn get_trail_canonical_combining_class_overlay<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_OVERLAY_V1)
}

pub fn get_trail_canonical_combining_class_ccc10<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC10_V1)
}

pub fn get_trail_canonical_combining_class_ccc103<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC103_V1)
}

pub fn get_trail_canonical_combining_class_ccc107<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC107_V1)
}

pub fn get_trail_canonical_combining_class_ccc11<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC11_V1)
}

pub fn get_trail_canonical_combining_class_ccc118<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC118_V1)
}

pub fn get_trail_canonical_combining_class_ccc12<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC12_V1)
}

pub fn get_trail_canonical_combining_class_ccc122<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC122_V1)
}

pub fn get_trail_canonical_combining_class_ccc129<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC129_V1)
}

pub fn get_trail_canonical_combining_class_ccc13<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC13_V1)
}

pub fn get_trail_canonical_combining_class_ccc130<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC130_V1)
}

pub fn get_trail_canonical_combining_class_ccc132<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC132_V1)
}

pub fn get_trail_canonical_combining_class_ccc133<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC133_V1)
}

pub fn get_trail_canonical_combining_class_ccc14<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC14_V1)
}

pub fn get_trail_canonical_combining_class_ccc15<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC15_V1)
}

pub fn get_trail_canonical_combining_class_ccc16<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC16_V1)
}

pub fn get_trail_canonical_combining_class_ccc17<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC17_V1)
}

pub fn get_trail_canonical_combining_class_ccc18<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC18_V1)
}

pub fn get_trail_canonical_combining_class_ccc19<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC19_V1)
}

pub fn get_trail_canonical_combining_class_ccc20<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC20_V1)
}

pub fn get_trail_canonical_combining_class_attached_below_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_LEFT_V1,
    )
}

pub fn get_trail_canonical_combining_class_attached_below<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_BELOW_V1,
    )
}

pub fn get_trail_canonical_combining_class_ccc21<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC21_V1)
}

pub fn get_trail_canonical_combining_class_attached_above<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_V1,
    )
}

pub fn get_trail_canonical_combining_class_attached_above_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_ATTACHED_ABOVE_RIGHT_V1,
    )
}

pub fn get_trail_canonical_combining_class_below_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_BELOW_LEFT_V1)
}

pub fn get_trail_canonical_combining_class_ccc22<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC22_V1)
}

pub fn get_trail_canonical_combining_class_below<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_BELOW_V1)
}

pub fn get_trail_canonical_combining_class_below_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_BELOW_RIGHT_V1,
    )
}

pub fn get_trail_canonical_combining_class_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_LEFT_V1)
}

pub fn get_trail_canonical_combining_class_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_RIGHT_V1)
}

pub fn get_trail_canonical_combining_class_above_left<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_LEFT_V1)
}

pub fn get_trail_canonical_combining_class_ccc23<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC23_V1)
}

pub fn get_trail_canonical_combining_class_above<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_V1)
}

pub fn get_trail_canonical_combining_class_above_right<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_ABOVE_RIGHT_V1,
    )
}

pub fn get_trail_canonical_combining_class_double_below<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_DOUBLE_BELOW_V1,
    )
}

pub fn get_trail_canonical_combining_class_double_above<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_DOUBLE_ABOVE_V1,
    )
}

pub fn get_trail_canonical_combining_class_ccc24<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC24_V1)
}

pub fn get_trail_canonical_combining_class_iota_subscript<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_IOTA_SUBSCRIPT_V1,
    )
}

pub fn get_trail_canonical_combining_class_ccc25<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC25_V1)
}

pub fn get_trail_canonical_combining_class_ccc26<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC26_V1)
}

pub fn get_trail_canonical_combining_class_ccc27<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC27_V1)
}

pub fn get_trail_canonical_combining_class_ccc28<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC28_V1)
}

pub fn get_trail_canonical_combining_class_ccc29<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC29_V1)
}

pub fn get_trail_canonical_combining_class_ccc30<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC30_V1)
}

pub fn get_trail_canonical_combining_class_ccc31<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC31_V1)
}

pub fn get_trail_canonical_combining_class_ccc32<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC32_V1)
}

pub fn get_trail_canonical_combining_class_ccc33<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC33_V1)
}

pub fn get_trail_canonical_combining_class_ccc34<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC34_V1)
}

pub fn get_trail_canonical_combining_class_ccc35<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC35_V1)
}

pub fn get_trail_canonical_combining_class_ccc36<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC36_V1)
}

pub fn get_trail_canonical_combining_class_han_reading<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_HAN_READING_V1,
    )
}

pub fn get_trail_canonical_combining_class_nukta<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_NUKTA_V1)
}

pub fn get_trail_canonical_combining_class_kana_voicing<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(
        provider,
        key::TRAIL_CANONICAL_COMBINING_CLASS_KANA_VOICING_V1,
    )
}

pub fn get_trail_canonical_combining_class_ccc84<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC84_V1)
}

pub fn get_trail_canonical_combining_class_virama<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_VIRAMA_V1)
}

pub fn get_trail_canonical_combining_class_ccc91<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::TRAIL_CANONICAL_COMBINING_CLASS_CCC91_V1)
}

pub fn get_vertical_orientation_rotated<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::VERTICAL_ORIENTATION_ROTATED_V1)
}

pub fn get_vertical_orientation_transformed_rotated<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::VERTICAL_ORIENTATION_TRANSFORMED_ROTATED_V1)
}

pub fn get_vertical_orientation_transformed_upright<
    'd,
    D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized,
>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::VERTICAL_ORIENTATION_TRANSFORMED_UPRIGHT_V1)
}

pub fn get_vertical_orientation_upright<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::VERTICAL_ORIENTATION_UPRIGHT_V1)
}

pub fn get_word_break_cr<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_CR_V1)
}

pub fn get_word_break_double_quote<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_DOUBLE_QUOTE_V1)
}

pub fn get_word_break_e_base<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_E_BASE_V1)
}

pub fn get_word_break_e_base_gaz<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_E_BASE_GAZ_V1)
}

pub fn get_word_break_e_modifier<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_E_MODIFIER_V1)
}

pub fn get_word_break_extendnumlet<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_EXTENDNUMLET_V1)
}

pub fn get_word_break_extend<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_EXTEND_V1)
}

pub fn get_word_break_format<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_FORMAT_V1)
}

pub fn get_word_break_glue_after_zwj<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_GLUE_AFTER_ZWJ_V1)
}

pub fn get_word_break_hebrew_letter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_HEBREW_LETTER_V1)
}

pub fn get_word_break_katakana<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_KATAKANA_V1)
}

pub fn get_word_break_aletter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_ALETTER_V1)
}

pub fn get_word_break_lf<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_LF_V1)
}

pub fn get_word_break_midnumlet<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_MIDNUMLET_V1)
}

pub fn get_word_break_midletter<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_MIDLETTER_V1)
}

pub fn get_word_break_midnum<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_MIDNUM_V1)
}

pub fn get_word_break_newline<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_NEWLINE_V1)
}

pub fn get_word_break_numeric<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_NUMERIC_V1)
}

pub fn get_word_break_regional_indicator<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_REGIONAL_INDICATOR_V1)
}

pub fn get_word_break_single_quote<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_SINGLE_QUOTE_V1)
}

pub fn get_word_break_wsegspace<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_WSEGSPACE_V1)
}

pub fn get_word_break_other<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_OTHER_V1)
}

pub fn get_word_break_zwj<'d, D: DataProvider<'d, UnicodeProperty<'d>> + ?Sized>(
    provider: &D,
) -> Result<UnicodeSet, UnicodeSetError> {
    get_prop(provider, key::WORD_BREAK_ZWJ_V1)
}
