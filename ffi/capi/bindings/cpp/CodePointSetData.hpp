#ifndef CodePointSetData_HPP
#define CodePointSetData_HPP

#include "CodePointSetData.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointRangeIterator.hpp"
#include "DataError.hpp"
#include "DataProvider.hpp"
#include "Error.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    bool icu4x_CodePointSetData_contains_mv1(const diplomat::capi::CodePointSetData* self, char32_t cp);
    
    diplomat::capi::CodePointRangeIterator* icu4x_CodePointSetData_iter_ranges_mv1(const diplomat::capi::CodePointSetData* self);
    
    diplomat::capi::CodePointRangeIterator* icu4x_CodePointSetData_iter_ranges_complemented_mv1(const diplomat::capi::CodePointSetData* self);
    
    typedef struct icu4x_CodePointSetData_load_for_general_category_group_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_for_general_category_group_mv1_result;
    icu4x_CodePointSetData_load_for_general_category_group_mv1_result icu4x_CodePointSetData_load_for_general_category_group_mv1(const diplomat::capi::DataProvider* provider, uint32_t group);
    
    typedef struct icu4x_CodePointSetData_load_ascii_hex_digit_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_ascii_hex_digit_mv1_result;
    icu4x_CodePointSetData_load_ascii_hex_digit_mv1_result icu4x_CodePointSetData_load_ascii_hex_digit_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_alnum_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_alnum_mv1_result;
    icu4x_CodePointSetData_load_alnum_mv1_result icu4x_CodePointSetData_load_alnum_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_alphabetic_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_alphabetic_mv1_result;
    icu4x_CodePointSetData_load_alphabetic_mv1_result icu4x_CodePointSetData_load_alphabetic_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_bidi_control_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_bidi_control_mv1_result;
    icu4x_CodePointSetData_load_bidi_control_mv1_result icu4x_CodePointSetData_load_bidi_control_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_bidi_mirrored_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_bidi_mirrored_mv1_result;
    icu4x_CodePointSetData_load_bidi_mirrored_mv1_result icu4x_CodePointSetData_load_bidi_mirrored_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_blank_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_blank_mv1_result;
    icu4x_CodePointSetData_load_blank_mv1_result icu4x_CodePointSetData_load_blank_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_cased_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_cased_mv1_result;
    icu4x_CodePointSetData_load_cased_mv1_result icu4x_CodePointSetData_load_cased_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_case_ignorable_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_case_ignorable_mv1_result;
    icu4x_CodePointSetData_load_case_ignorable_mv1_result icu4x_CodePointSetData_load_case_ignorable_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_full_composition_exclusion_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_full_composition_exclusion_mv1_result;
    icu4x_CodePointSetData_load_full_composition_exclusion_mv1_result icu4x_CodePointSetData_load_full_composition_exclusion_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_changes_when_casefolded_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_changes_when_casefolded_mv1_result;
    icu4x_CodePointSetData_load_changes_when_casefolded_mv1_result icu4x_CodePointSetData_load_changes_when_casefolded_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_changes_when_casemapped_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_changes_when_casemapped_mv1_result;
    icu4x_CodePointSetData_load_changes_when_casemapped_mv1_result icu4x_CodePointSetData_load_changes_when_casemapped_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_changes_when_nfkc_casefolded_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_changes_when_nfkc_casefolded_mv1_result;
    icu4x_CodePointSetData_load_changes_when_nfkc_casefolded_mv1_result icu4x_CodePointSetData_load_changes_when_nfkc_casefolded_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_changes_when_lowercased_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_changes_when_lowercased_mv1_result;
    icu4x_CodePointSetData_load_changes_when_lowercased_mv1_result icu4x_CodePointSetData_load_changes_when_lowercased_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_changes_when_titlecased_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_changes_when_titlecased_mv1_result;
    icu4x_CodePointSetData_load_changes_when_titlecased_mv1_result icu4x_CodePointSetData_load_changes_when_titlecased_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_changes_when_uppercased_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_changes_when_uppercased_mv1_result;
    icu4x_CodePointSetData_load_changes_when_uppercased_mv1_result icu4x_CodePointSetData_load_changes_when_uppercased_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_dash_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_dash_mv1_result;
    icu4x_CodePointSetData_load_dash_mv1_result icu4x_CodePointSetData_load_dash_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_deprecated_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_deprecated_mv1_result;
    icu4x_CodePointSetData_load_deprecated_mv1_result icu4x_CodePointSetData_load_deprecated_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_default_ignorable_code_point_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_default_ignorable_code_point_mv1_result;
    icu4x_CodePointSetData_load_default_ignorable_code_point_mv1_result icu4x_CodePointSetData_load_default_ignorable_code_point_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_diacritic_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_diacritic_mv1_result;
    icu4x_CodePointSetData_load_diacritic_mv1_result icu4x_CodePointSetData_load_diacritic_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_emoji_modifier_base_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_emoji_modifier_base_mv1_result;
    icu4x_CodePointSetData_load_emoji_modifier_base_mv1_result icu4x_CodePointSetData_load_emoji_modifier_base_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_emoji_component_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_emoji_component_mv1_result;
    icu4x_CodePointSetData_load_emoji_component_mv1_result icu4x_CodePointSetData_load_emoji_component_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_emoji_modifier_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_emoji_modifier_mv1_result;
    icu4x_CodePointSetData_load_emoji_modifier_mv1_result icu4x_CodePointSetData_load_emoji_modifier_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_emoji_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_emoji_mv1_result;
    icu4x_CodePointSetData_load_emoji_mv1_result icu4x_CodePointSetData_load_emoji_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_emoji_presentation_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_emoji_presentation_mv1_result;
    icu4x_CodePointSetData_load_emoji_presentation_mv1_result icu4x_CodePointSetData_load_emoji_presentation_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_extender_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_extender_mv1_result;
    icu4x_CodePointSetData_load_extender_mv1_result icu4x_CodePointSetData_load_extender_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_extended_pictographic_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_extended_pictographic_mv1_result;
    icu4x_CodePointSetData_load_extended_pictographic_mv1_result icu4x_CodePointSetData_load_extended_pictographic_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_graph_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_graph_mv1_result;
    icu4x_CodePointSetData_load_graph_mv1_result icu4x_CodePointSetData_load_graph_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_grapheme_base_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_grapheme_base_mv1_result;
    icu4x_CodePointSetData_load_grapheme_base_mv1_result icu4x_CodePointSetData_load_grapheme_base_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_grapheme_extend_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_grapheme_extend_mv1_result;
    icu4x_CodePointSetData_load_grapheme_extend_mv1_result icu4x_CodePointSetData_load_grapheme_extend_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_grapheme_link_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_grapheme_link_mv1_result;
    icu4x_CodePointSetData_load_grapheme_link_mv1_result icu4x_CodePointSetData_load_grapheme_link_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_hex_digit_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_hex_digit_mv1_result;
    icu4x_CodePointSetData_load_hex_digit_mv1_result icu4x_CodePointSetData_load_hex_digit_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_hyphen_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_hyphen_mv1_result;
    icu4x_CodePointSetData_load_hyphen_mv1_result icu4x_CodePointSetData_load_hyphen_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_id_continue_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_id_continue_mv1_result;
    icu4x_CodePointSetData_load_id_continue_mv1_result icu4x_CodePointSetData_load_id_continue_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_ideographic_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_ideographic_mv1_result;
    icu4x_CodePointSetData_load_ideographic_mv1_result icu4x_CodePointSetData_load_ideographic_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_id_start_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_id_start_mv1_result;
    icu4x_CodePointSetData_load_id_start_mv1_result icu4x_CodePointSetData_load_id_start_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_ids_binary_operator_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_ids_binary_operator_mv1_result;
    icu4x_CodePointSetData_load_ids_binary_operator_mv1_result icu4x_CodePointSetData_load_ids_binary_operator_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_ids_trinary_operator_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_ids_trinary_operator_mv1_result;
    icu4x_CodePointSetData_load_ids_trinary_operator_mv1_result icu4x_CodePointSetData_load_ids_trinary_operator_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_join_control_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_join_control_mv1_result;
    icu4x_CodePointSetData_load_join_control_mv1_result icu4x_CodePointSetData_load_join_control_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_logical_order_exception_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_logical_order_exception_mv1_result;
    icu4x_CodePointSetData_load_logical_order_exception_mv1_result icu4x_CodePointSetData_load_logical_order_exception_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_lowercase_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_lowercase_mv1_result;
    icu4x_CodePointSetData_load_lowercase_mv1_result icu4x_CodePointSetData_load_lowercase_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_math_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_math_mv1_result;
    icu4x_CodePointSetData_load_math_mv1_result icu4x_CodePointSetData_load_math_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_noncharacter_code_point_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_noncharacter_code_point_mv1_result;
    icu4x_CodePointSetData_load_noncharacter_code_point_mv1_result icu4x_CodePointSetData_load_noncharacter_code_point_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_nfc_inert_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_nfc_inert_mv1_result;
    icu4x_CodePointSetData_load_nfc_inert_mv1_result icu4x_CodePointSetData_load_nfc_inert_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_nfd_inert_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_nfd_inert_mv1_result;
    icu4x_CodePointSetData_load_nfd_inert_mv1_result icu4x_CodePointSetData_load_nfd_inert_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_nfkc_inert_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_nfkc_inert_mv1_result;
    icu4x_CodePointSetData_load_nfkc_inert_mv1_result icu4x_CodePointSetData_load_nfkc_inert_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_nfkd_inert_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_nfkd_inert_mv1_result;
    icu4x_CodePointSetData_load_nfkd_inert_mv1_result icu4x_CodePointSetData_load_nfkd_inert_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_pattern_syntax_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_pattern_syntax_mv1_result;
    icu4x_CodePointSetData_load_pattern_syntax_mv1_result icu4x_CodePointSetData_load_pattern_syntax_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_pattern_white_space_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_pattern_white_space_mv1_result;
    icu4x_CodePointSetData_load_pattern_white_space_mv1_result icu4x_CodePointSetData_load_pattern_white_space_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_prepended_concatenation_mark_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_prepended_concatenation_mark_mv1_result;
    icu4x_CodePointSetData_load_prepended_concatenation_mark_mv1_result icu4x_CodePointSetData_load_prepended_concatenation_mark_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_print_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_print_mv1_result;
    icu4x_CodePointSetData_load_print_mv1_result icu4x_CodePointSetData_load_print_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_quotation_mark_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_quotation_mark_mv1_result;
    icu4x_CodePointSetData_load_quotation_mark_mv1_result icu4x_CodePointSetData_load_quotation_mark_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_radical_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_radical_mv1_result;
    icu4x_CodePointSetData_load_radical_mv1_result icu4x_CodePointSetData_load_radical_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_regional_indicator_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_regional_indicator_mv1_result;
    icu4x_CodePointSetData_load_regional_indicator_mv1_result icu4x_CodePointSetData_load_regional_indicator_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_soft_dotted_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_soft_dotted_mv1_result;
    icu4x_CodePointSetData_load_soft_dotted_mv1_result icu4x_CodePointSetData_load_soft_dotted_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_segment_starter_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_segment_starter_mv1_result;
    icu4x_CodePointSetData_load_segment_starter_mv1_result icu4x_CodePointSetData_load_segment_starter_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_case_sensitive_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_case_sensitive_mv1_result;
    icu4x_CodePointSetData_load_case_sensitive_mv1_result icu4x_CodePointSetData_load_case_sensitive_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_sentence_terminal_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_sentence_terminal_mv1_result;
    icu4x_CodePointSetData_load_sentence_terminal_mv1_result icu4x_CodePointSetData_load_sentence_terminal_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_terminal_punctuation_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_terminal_punctuation_mv1_result;
    icu4x_CodePointSetData_load_terminal_punctuation_mv1_result icu4x_CodePointSetData_load_terminal_punctuation_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_unified_ideograph_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_unified_ideograph_mv1_result;
    icu4x_CodePointSetData_load_unified_ideograph_mv1_result icu4x_CodePointSetData_load_unified_ideograph_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_uppercase_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_uppercase_mv1_result;
    icu4x_CodePointSetData_load_uppercase_mv1_result icu4x_CodePointSetData_load_uppercase_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_variation_selector_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_variation_selector_mv1_result;
    icu4x_CodePointSetData_load_variation_selector_mv1_result icu4x_CodePointSetData_load_variation_selector_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_white_space_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_white_space_mv1_result;
    icu4x_CodePointSetData_load_white_space_mv1_result icu4x_CodePointSetData_load_white_space_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_xdigit_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_xdigit_mv1_result;
    icu4x_CodePointSetData_load_xdigit_mv1_result icu4x_CodePointSetData_load_xdigit_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_xid_continue_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_xid_continue_mv1_result;
    icu4x_CodePointSetData_load_xid_continue_mv1_result icu4x_CodePointSetData_load_xid_continue_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_xid_start_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::DataError err;}; bool is_ok;} icu4x_CodePointSetData_load_xid_start_mv1_result;
    icu4x_CodePointSetData_load_xid_start_mv1_result icu4x_CodePointSetData_load_xid_start_mv1(const diplomat::capi::DataProvider* provider);
    
    typedef struct icu4x_CodePointSetData_load_for_ecma262_mv1_result {union {diplomat::capi::CodePointSetData* ok; diplomat::capi::Error err;}; bool is_ok;} icu4x_CodePointSetData_load_for_ecma262_mv1_result;
    icu4x_CodePointSetData_load_for_ecma262_mv1_result icu4x_CodePointSetData_load_for_ecma262_mv1(const diplomat::capi::DataProvider* provider, const char* property_name_data, size_t property_name_len);
    
    
    void icu4x_CodePointSetData_destroy_mv1(CodePointSetData* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline bool CodePointSetData::contains(char32_t cp) const {
  auto result = diplomat::capi::icu4x_CodePointSetData_contains_mv1(this->AsFFI(),
    cp);
  return result;
}

inline std::unique_ptr<CodePointRangeIterator> CodePointSetData::iter_ranges() const {
  auto result = diplomat::capi::icu4x_CodePointSetData_iter_ranges_mv1(this->AsFFI());
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> CodePointSetData::iter_ranges_complemented() const {
  auto result = diplomat::capi::icu4x_CodePointSetData_iter_ranges_complemented_mv1(this->AsFFI());
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_for_general_category_group(const DataProvider& provider, uint32_t group) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_for_general_category_group_mv1(provider.AsFFI(),
    group);
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_ascii_hex_digit(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_ascii_hex_digit_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_alnum(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_alnum_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_alphabetic(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_alphabetic_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_bidi_control(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_bidi_control_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_bidi_mirrored(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_bidi_mirrored_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_blank(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_blank_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_cased(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_cased_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_case_ignorable(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_case_ignorable_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_full_composition_exclusion(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_full_composition_exclusion_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_casefolded(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_changes_when_casefolded_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_casemapped(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_changes_when_casemapped_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_nfkc_casefolded(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_changes_when_nfkc_casefolded_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_lowercased(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_changes_when_lowercased_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_titlecased(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_changes_when_titlecased_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_uppercased(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_changes_when_uppercased_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_dash(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_dash_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_deprecated(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_deprecated_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_default_ignorable_code_point(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_default_ignorable_code_point_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_diacritic(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_diacritic_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji_modifier_base(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_emoji_modifier_base_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji_component(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_emoji_component_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji_modifier(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_emoji_modifier_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_emoji_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji_presentation(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_emoji_presentation_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_extender(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_extender_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_extended_pictographic(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_extended_pictographic_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_graph(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_graph_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_grapheme_base(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_grapheme_base_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_grapheme_extend(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_grapheme_extend_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_grapheme_link(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_grapheme_link_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_hex_digit(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_hex_digit_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_hyphen(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_hyphen_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_id_continue(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_id_continue_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_ideographic(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_ideographic_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_id_start(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_id_start_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_ids_binary_operator(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_ids_binary_operator_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_ids_trinary_operator(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_ids_trinary_operator_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_join_control(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_join_control_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_logical_order_exception(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_logical_order_exception_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_lowercase(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_lowercase_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_math(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_math_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_noncharacter_code_point(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_noncharacter_code_point_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_nfc_inert(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_nfc_inert_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_nfd_inert(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_nfd_inert_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_nfkc_inert(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_nfkc_inert_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_nfkd_inert(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_nfkd_inert_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_pattern_syntax(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_pattern_syntax_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_pattern_white_space(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_pattern_white_space_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_prepended_concatenation_mark(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_prepended_concatenation_mark_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_print(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_print_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_quotation_mark(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_quotation_mark_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_radical(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_radical_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_regional_indicator(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_regional_indicator_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_soft_dotted(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_soft_dotted_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_segment_starter(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_segment_starter_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_case_sensitive(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_case_sensitive_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_sentence_terminal(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_sentence_terminal_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_terminal_punctuation(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_terminal_punctuation_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_unified_ideograph(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_unified_ideograph_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_uppercase(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_uppercase_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_variation_selector(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_variation_selector_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_white_space(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_white_space_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_xdigit(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_xdigit_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_xid_continue(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_xid_continue_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_xid_start(const DataProvider& provider) {
  auto result = diplomat::capi::icu4x_CodePointSetData_load_xid_start_mv1(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<diplomat::result<std::unique_ptr<CodePointSetData>, Error>, diplomat::Utf8Error> CodePointSetData::load_for_ecma262(const DataProvider& provider, std::string_view property_name) {
  if (!diplomat::capi::diplomat_is_str(property_name.data(), property_name.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  auto result = diplomat::capi::icu4x_CodePointSetData_load_for_ecma262_mv1(provider.AsFFI(),
    property_name.data(),
    property_name.size());
  return diplomat::Ok<diplomat::result<std::unique_ptr<CodePointSetData>, Error>>(result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, Error>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err))));
}

inline const diplomat::capi::CodePointSetData* CodePointSetData::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CodePointSetData*>(this);
}

inline diplomat::capi::CodePointSetData* CodePointSetData::AsFFI() {
  return reinterpret_cast<diplomat::capi::CodePointSetData*>(this);
}

inline const CodePointSetData* CodePointSetData::FromFFI(const diplomat::capi::CodePointSetData* ptr) {
  return reinterpret_cast<const CodePointSetData*>(ptr);
}

inline CodePointSetData* CodePointSetData::FromFFI(diplomat::capi::CodePointSetData* ptr) {
  return reinterpret_cast<CodePointSetData*>(ptr);
}

inline void CodePointSetData::operator delete(void* ptr) {
  diplomat::capi::icu4x_CodePointSetData_destroy_mv1(reinterpret_cast<diplomat::capi::CodePointSetData*>(ptr));
}


#endif // CodePointSetData_HPP
