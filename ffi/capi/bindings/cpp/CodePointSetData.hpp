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


namespace capi {
    extern "C" {
    
    bool ICU4XCodePointSetData_contains(const CodePointSetData* self, char32_t cp);
    
    bool ICU4XCodePointSetData_contains32(const CodePointSetData* self, uint32_t cp);
    
    CodePointRangeIterator* ICU4XCodePointSetData_iter_ranges(const CodePointSetData* self);
    
    CodePointRangeIterator* ICU4XCodePointSetData_iter_ranges_complemented(const CodePointSetData* self);
    
    typedef struct ICU4XCodePointSetData_load_for_general_category_group_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_for_general_category_group_result;
    ICU4XCodePointSetData_load_for_general_category_group_result ICU4XCodePointSetData_load_for_general_category_group(const DataProvider* provider, uint32_t group);
    
    typedef struct ICU4XCodePointSetData_load_ascii_hex_digit_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_ascii_hex_digit_result;
    ICU4XCodePointSetData_load_ascii_hex_digit_result ICU4XCodePointSetData_load_ascii_hex_digit(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_alnum_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_alnum_result;
    ICU4XCodePointSetData_load_alnum_result ICU4XCodePointSetData_load_alnum(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_alphabetic_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_alphabetic_result;
    ICU4XCodePointSetData_load_alphabetic_result ICU4XCodePointSetData_load_alphabetic(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_bidi_control_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_bidi_control_result;
    ICU4XCodePointSetData_load_bidi_control_result ICU4XCodePointSetData_load_bidi_control(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_bidi_mirrored_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_bidi_mirrored_result;
    ICU4XCodePointSetData_load_bidi_mirrored_result ICU4XCodePointSetData_load_bidi_mirrored(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_blank_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_blank_result;
    ICU4XCodePointSetData_load_blank_result ICU4XCodePointSetData_load_blank(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_cased_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_cased_result;
    ICU4XCodePointSetData_load_cased_result ICU4XCodePointSetData_load_cased(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_case_ignorable_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_case_ignorable_result;
    ICU4XCodePointSetData_load_case_ignorable_result ICU4XCodePointSetData_load_case_ignorable(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_full_composition_exclusion_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_full_composition_exclusion_result;
    ICU4XCodePointSetData_load_full_composition_exclusion_result ICU4XCodePointSetData_load_full_composition_exclusion(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_changes_when_casefolded_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_changes_when_casefolded_result;
    ICU4XCodePointSetData_load_changes_when_casefolded_result ICU4XCodePointSetData_load_changes_when_casefolded(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_changes_when_casemapped_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_changes_when_casemapped_result;
    ICU4XCodePointSetData_load_changes_when_casemapped_result ICU4XCodePointSetData_load_changes_when_casemapped(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_changes_when_nfkc_casefolded_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_changes_when_nfkc_casefolded_result;
    ICU4XCodePointSetData_load_changes_when_nfkc_casefolded_result ICU4XCodePointSetData_load_changes_when_nfkc_casefolded(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_changes_when_lowercased_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_changes_when_lowercased_result;
    ICU4XCodePointSetData_load_changes_when_lowercased_result ICU4XCodePointSetData_load_changes_when_lowercased(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_changes_when_titlecased_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_changes_when_titlecased_result;
    ICU4XCodePointSetData_load_changes_when_titlecased_result ICU4XCodePointSetData_load_changes_when_titlecased(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_changes_when_uppercased_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_changes_when_uppercased_result;
    ICU4XCodePointSetData_load_changes_when_uppercased_result ICU4XCodePointSetData_load_changes_when_uppercased(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_dash_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_dash_result;
    ICU4XCodePointSetData_load_dash_result ICU4XCodePointSetData_load_dash(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_deprecated_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_deprecated_result;
    ICU4XCodePointSetData_load_deprecated_result ICU4XCodePointSetData_load_deprecated(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_default_ignorable_code_point_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_default_ignorable_code_point_result;
    ICU4XCodePointSetData_load_default_ignorable_code_point_result ICU4XCodePointSetData_load_default_ignorable_code_point(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_diacritic_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_diacritic_result;
    ICU4XCodePointSetData_load_diacritic_result ICU4XCodePointSetData_load_diacritic(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_emoji_modifier_base_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_emoji_modifier_base_result;
    ICU4XCodePointSetData_load_emoji_modifier_base_result ICU4XCodePointSetData_load_emoji_modifier_base(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_emoji_component_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_emoji_component_result;
    ICU4XCodePointSetData_load_emoji_component_result ICU4XCodePointSetData_load_emoji_component(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_emoji_modifier_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_emoji_modifier_result;
    ICU4XCodePointSetData_load_emoji_modifier_result ICU4XCodePointSetData_load_emoji_modifier(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_emoji_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_emoji_result;
    ICU4XCodePointSetData_load_emoji_result ICU4XCodePointSetData_load_emoji(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_emoji_presentation_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_emoji_presentation_result;
    ICU4XCodePointSetData_load_emoji_presentation_result ICU4XCodePointSetData_load_emoji_presentation(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_extender_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_extender_result;
    ICU4XCodePointSetData_load_extender_result ICU4XCodePointSetData_load_extender(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_extended_pictographic_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_extended_pictographic_result;
    ICU4XCodePointSetData_load_extended_pictographic_result ICU4XCodePointSetData_load_extended_pictographic(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_graph_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_graph_result;
    ICU4XCodePointSetData_load_graph_result ICU4XCodePointSetData_load_graph(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_grapheme_base_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_grapheme_base_result;
    ICU4XCodePointSetData_load_grapheme_base_result ICU4XCodePointSetData_load_grapheme_base(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_grapheme_extend_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_grapheme_extend_result;
    ICU4XCodePointSetData_load_grapheme_extend_result ICU4XCodePointSetData_load_grapheme_extend(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_grapheme_link_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_grapheme_link_result;
    ICU4XCodePointSetData_load_grapheme_link_result ICU4XCodePointSetData_load_grapheme_link(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_hex_digit_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_hex_digit_result;
    ICU4XCodePointSetData_load_hex_digit_result ICU4XCodePointSetData_load_hex_digit(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_hyphen_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_hyphen_result;
    ICU4XCodePointSetData_load_hyphen_result ICU4XCodePointSetData_load_hyphen(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_id_continue_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_id_continue_result;
    ICU4XCodePointSetData_load_id_continue_result ICU4XCodePointSetData_load_id_continue(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_ideographic_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_ideographic_result;
    ICU4XCodePointSetData_load_ideographic_result ICU4XCodePointSetData_load_ideographic(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_id_start_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_id_start_result;
    ICU4XCodePointSetData_load_id_start_result ICU4XCodePointSetData_load_id_start(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_ids_binary_operator_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_ids_binary_operator_result;
    ICU4XCodePointSetData_load_ids_binary_operator_result ICU4XCodePointSetData_load_ids_binary_operator(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_ids_trinary_operator_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_ids_trinary_operator_result;
    ICU4XCodePointSetData_load_ids_trinary_operator_result ICU4XCodePointSetData_load_ids_trinary_operator(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_join_control_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_join_control_result;
    ICU4XCodePointSetData_load_join_control_result ICU4XCodePointSetData_load_join_control(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_logical_order_exception_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_logical_order_exception_result;
    ICU4XCodePointSetData_load_logical_order_exception_result ICU4XCodePointSetData_load_logical_order_exception(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_lowercase_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_lowercase_result;
    ICU4XCodePointSetData_load_lowercase_result ICU4XCodePointSetData_load_lowercase(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_math_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_math_result;
    ICU4XCodePointSetData_load_math_result ICU4XCodePointSetData_load_math(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_noncharacter_code_point_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_noncharacter_code_point_result;
    ICU4XCodePointSetData_load_noncharacter_code_point_result ICU4XCodePointSetData_load_noncharacter_code_point(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_nfc_inert_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_nfc_inert_result;
    ICU4XCodePointSetData_load_nfc_inert_result ICU4XCodePointSetData_load_nfc_inert(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_nfd_inert_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_nfd_inert_result;
    ICU4XCodePointSetData_load_nfd_inert_result ICU4XCodePointSetData_load_nfd_inert(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_nfkc_inert_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_nfkc_inert_result;
    ICU4XCodePointSetData_load_nfkc_inert_result ICU4XCodePointSetData_load_nfkc_inert(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_nfkd_inert_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_nfkd_inert_result;
    ICU4XCodePointSetData_load_nfkd_inert_result ICU4XCodePointSetData_load_nfkd_inert(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_pattern_syntax_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_pattern_syntax_result;
    ICU4XCodePointSetData_load_pattern_syntax_result ICU4XCodePointSetData_load_pattern_syntax(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_pattern_white_space_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_pattern_white_space_result;
    ICU4XCodePointSetData_load_pattern_white_space_result ICU4XCodePointSetData_load_pattern_white_space(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_prepended_concatenation_mark_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_prepended_concatenation_mark_result;
    ICU4XCodePointSetData_load_prepended_concatenation_mark_result ICU4XCodePointSetData_load_prepended_concatenation_mark(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_print_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_print_result;
    ICU4XCodePointSetData_load_print_result ICU4XCodePointSetData_load_print(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_quotation_mark_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_quotation_mark_result;
    ICU4XCodePointSetData_load_quotation_mark_result ICU4XCodePointSetData_load_quotation_mark(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_radical_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_radical_result;
    ICU4XCodePointSetData_load_radical_result ICU4XCodePointSetData_load_radical(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_regional_indicator_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_regional_indicator_result;
    ICU4XCodePointSetData_load_regional_indicator_result ICU4XCodePointSetData_load_regional_indicator(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_soft_dotted_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_soft_dotted_result;
    ICU4XCodePointSetData_load_soft_dotted_result ICU4XCodePointSetData_load_soft_dotted(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_segment_starter_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_segment_starter_result;
    ICU4XCodePointSetData_load_segment_starter_result ICU4XCodePointSetData_load_segment_starter(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_case_sensitive_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_case_sensitive_result;
    ICU4XCodePointSetData_load_case_sensitive_result ICU4XCodePointSetData_load_case_sensitive(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_sentence_terminal_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_sentence_terminal_result;
    ICU4XCodePointSetData_load_sentence_terminal_result ICU4XCodePointSetData_load_sentence_terminal(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_terminal_punctuation_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_terminal_punctuation_result;
    ICU4XCodePointSetData_load_terminal_punctuation_result ICU4XCodePointSetData_load_terminal_punctuation(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_unified_ideograph_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_unified_ideograph_result;
    ICU4XCodePointSetData_load_unified_ideograph_result ICU4XCodePointSetData_load_unified_ideograph(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_uppercase_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_uppercase_result;
    ICU4XCodePointSetData_load_uppercase_result ICU4XCodePointSetData_load_uppercase(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_variation_selector_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_variation_selector_result;
    ICU4XCodePointSetData_load_variation_selector_result ICU4XCodePointSetData_load_variation_selector(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_white_space_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_white_space_result;
    ICU4XCodePointSetData_load_white_space_result ICU4XCodePointSetData_load_white_space(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_xdigit_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_xdigit_result;
    ICU4XCodePointSetData_load_xdigit_result ICU4XCodePointSetData_load_xdigit(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_xid_continue_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_xid_continue_result;
    ICU4XCodePointSetData_load_xid_continue_result ICU4XCodePointSetData_load_xid_continue(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_xid_start_result {union {CodePointSetData* ok; DataError err;}; bool is_ok;} ICU4XCodePointSetData_load_xid_start_result;
    ICU4XCodePointSetData_load_xid_start_result ICU4XCodePointSetData_load_xid_start(const DataProvider* provider);
    
    typedef struct ICU4XCodePointSetData_load_for_ecma262_result {union {CodePointSetData* ok; Error err;}; bool is_ok;} ICU4XCodePointSetData_load_for_ecma262_result;
    ICU4XCodePointSetData_load_for_ecma262_result ICU4XCodePointSetData_load_for_ecma262(const DataProvider* provider, const char* property_name_data, size_t property_name_len);
    
    
    void ICU4XCodePointSetData_destroy(CodePointSetData* self);
    
    } // extern "C"
}

inline bool CodePointSetData::contains(char32_t cp) const {
  auto result = capi::ICU4XCodePointSetData_contains(this->AsFFI(),
    cp);
  return result;
}

inline bool CodePointSetData::contains32(uint32_t cp) const {
  auto result = capi::ICU4XCodePointSetData_contains32(this->AsFFI(),
    cp);
  return result;
}

inline std::unique_ptr<CodePointRangeIterator> CodePointSetData::iter_ranges() const {
  auto result = capi::ICU4XCodePointSetData_iter_ranges(this->AsFFI());
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<CodePointRangeIterator> CodePointSetData::iter_ranges_complemented() const {
  auto result = capi::ICU4XCodePointSetData_iter_ranges_complemented(this->AsFFI());
  return std::unique_ptr<CodePointRangeIterator>(CodePointRangeIterator::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_for_general_category_group(const DataProvider& provider, uint32_t group) {
  auto result = capi::ICU4XCodePointSetData_load_for_general_category_group(provider.AsFFI(),
    group);
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_ascii_hex_digit(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_ascii_hex_digit(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_alnum(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_alnum(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_alphabetic(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_alphabetic(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_bidi_control(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_bidi_control(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_bidi_mirrored(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_bidi_mirrored(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_blank(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_blank(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_cased(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_cased(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_case_ignorable(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_case_ignorable(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_full_composition_exclusion(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_full_composition_exclusion(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_casefolded(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_casefolded(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_casemapped(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_casemapped(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_nfkc_casefolded(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_nfkc_casefolded(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_lowercased(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_lowercased(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_titlecased(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_titlecased(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_changes_when_uppercased(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_changes_when_uppercased(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_dash(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_dash(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_deprecated(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_deprecated(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_default_ignorable_code_point(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_default_ignorable_code_point(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_diacritic(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_diacritic(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji_modifier_base(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji_modifier_base(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji_component(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji_component(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji_modifier(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji_modifier(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_emoji_presentation(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_emoji_presentation(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_extender(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_extender(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_extended_pictographic(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_extended_pictographic(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_graph(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_graph(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_grapheme_base(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_grapheme_base(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_grapheme_extend(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_grapheme_extend(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_grapheme_link(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_grapheme_link(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_hex_digit(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_hex_digit(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_hyphen(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_hyphen(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_id_continue(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_id_continue(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_ideographic(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_ideographic(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_id_start(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_id_start(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_ids_binary_operator(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_ids_binary_operator(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_ids_trinary_operator(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_ids_trinary_operator(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_join_control(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_join_control(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_logical_order_exception(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_logical_order_exception(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_lowercase(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_lowercase(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_math(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_math(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_noncharacter_code_point(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_noncharacter_code_point(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_nfc_inert(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_nfc_inert(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_nfd_inert(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_nfd_inert(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_nfkc_inert(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_nfkc_inert(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_nfkd_inert(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_nfkd_inert(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_pattern_syntax(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_pattern_syntax(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_pattern_white_space(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_pattern_white_space(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_prepended_concatenation_mark(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_prepended_concatenation_mark(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_print(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_print(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_quotation_mark(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_quotation_mark(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_radical(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_radical(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_regional_indicator(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_regional_indicator(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_soft_dotted(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_soft_dotted(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_segment_starter(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_segment_starter(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_case_sensitive(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_case_sensitive(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_sentence_terminal(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_sentence_terminal(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_terminal_punctuation(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_terminal_punctuation(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_unified_ideograph(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_unified_ideograph(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_uppercase(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_uppercase(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_variation_selector(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_variation_selector(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_white_space(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_white_space(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_xdigit(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_xdigit(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_xid_continue(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_xid_continue(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<CodePointSetData>, DataError> CodePointSetData::load_xid_start(const DataProvider& provider) {
  auto result = capi::ICU4XCodePointSetData_load_xid_start(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, DataError>(diplomat::Err<DataError>(DataError::FromFFI(result.err)));
}

inline diplomat::result<diplomat::result<std::unique_ptr<CodePointSetData>, Error>, diplomat::Utf8Error> CodePointSetData::load_for_ecma262(const DataProvider& provider, std::string_view property_name) {
  if (!capi::diplomat_is_str(property_name.data(), property_name.size())) {
    return diplomat::Err<diplomat::Utf8Error>(diplomat::Utf8Error());
  }
  auto result = capi::ICU4XCodePointSetData_load_for_ecma262(provider.AsFFI(),
    property_name.data(),
    property_name.size());
  return diplomat::Ok<diplomat::result<std::unique_ptr<CodePointSetData>, Error>>(std::move(result.is_ok ? diplomat::result<std::unique_ptr<CodePointSetData>, Error>(diplomat::Ok<std::unique_ptr<CodePointSetData>>(std::unique_ptr<CodePointSetData>(CodePointSetData::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<CodePointSetData>, Error>(diplomat::Err<Error>(Error::FromFFI(result.err)))));
}

inline const capi::CodePointSetData* CodePointSetData::AsFFI() const {
  return reinterpret_cast<const capi::CodePointSetData*>(this);
}

inline capi::CodePointSetData* CodePointSetData::AsFFI() {
  return reinterpret_cast<capi::CodePointSetData*>(this);
}

inline const CodePointSetData* CodePointSetData::FromFFI(const capi::CodePointSetData* ptr) {
  return reinterpret_cast<const CodePointSetData*>(ptr);
}

inline CodePointSetData* CodePointSetData::FromFFI(capi::CodePointSetData* ptr) {
  return reinterpret_cast<CodePointSetData*>(ptr);
}

inline void CodePointSetData::operator delete(void* ptr) {
  capi::ICU4XCodePointSetData_destroy(reinterpret_cast<capi::CodePointSetData*>(ptr));
}


#endif // CodePointSetData_HPP
