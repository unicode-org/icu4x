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
