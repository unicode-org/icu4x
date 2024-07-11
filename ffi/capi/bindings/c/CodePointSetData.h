#ifndef CodePointSetData_H
#define CodePointSetData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CodePointRangeIterator.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Error.d.h"

#include "CodePointSetData.d.h"






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





#endif // CodePointSetData_H
