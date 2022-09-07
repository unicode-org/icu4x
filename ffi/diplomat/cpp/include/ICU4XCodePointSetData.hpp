#ifndef ICU4XCodePointSetData_HPP
#define ICU4XCodePointSetData_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCodePointSetData.h"

class ICU4XDataProvider;
class ICU4XCodePointSetData;
#include "ICU4XError.hpp"

/**
 * A destruction policy for using ICU4XCodePointSetData with std::unique_ptr.
 */
struct ICU4XCodePointSetDataDeleter {
  void operator()(capi::ICU4XCodePointSetData* l) const noexcept {
    capi::ICU4XCodePointSetData_destroy(l);
  }
};

/**
 * An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
 * 
 * See the [Rust documentation for `properties`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/index.html) for more information.
 * 
 * See the [Rust documentation for `CodePointSetData`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetData.html) for more information.
 * 
 * See the [Rust documentation for `CodePointSetDataBorrowed`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html) for more information.
 */
class ICU4XCodePointSetData {
 public:

  /**
   * Checks whether the code point is in the set.
   * 
   * See the [Rust documentation for `contains`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.contains) for more information.
   */
  bool contains(char32_t cp) const;

  /**
   * Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.
   */
  bool contains32(uint32_t cp) const;

  /**
   * which is a mask with the same format as the `U_GC_XX_MASK` mask in ICU4C
   * 
   * See the [Rust documentation for `load_for_general_category_group`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_for_general_category_group.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_for_general_category_group(const ICU4XDataProvider& provider, uint32_t group);

  /**
   * 
   * 
   * See the [Rust documentation for `load_ascii_hex_digit`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ascii_hex_digit.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ascii_hex_digit(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_alnum`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_alnum.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_alnum(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_alphabetic`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_alphabetic.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_alphabetic(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_bidi_control`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_bidi_control.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_bidi_control(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_bidi_mirrored`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_bidi_mirrored.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_bidi_mirrored(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_blank`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_blank.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_blank(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_cased`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_cased.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_cased(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_case_ignorable`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_case_ignorable.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_case_ignorable(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_full_composition_exclusion`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_full_composition_exclusion.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_full_composition_exclusion(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_changes_when_casefolded`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_casefolded.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_casefolded(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_changes_when_casemapped`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_casemapped.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_casemapped(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_changes_when_nfkc_casefolded`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_nfkc_casefolded.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_nfkc_casefolded(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_changes_when_lowercased`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_lowercased.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_lowercased(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_changes_when_titlecased`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_titlecased.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_titlecased(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_changes_when_uppercased`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_changes_when_uppercased.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_changes_when_uppercased(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_dash`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_dash.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_dash(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_deprecated`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_deprecated.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_deprecated(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_default_ignorable_code_point`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_default_ignorable_code_point.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_default_ignorable_code_point(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_diacritic`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_diacritic.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_diacritic(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_emoji_modifier_base`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_modifier_base.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_modifier_base(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_emoji_component`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_component.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_component(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_emoji_modifier`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_modifier.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_modifier(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_emoji`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_emoji_presentation`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_emoji_presentation.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_emoji_presentation(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_extender`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_extender.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_extender(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_extended_pictographic`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_extended_pictographic.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_extended_pictographic(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_graph`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_graph.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_graph(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_grapheme_base`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_grapheme_base.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_grapheme_base(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_grapheme_extend`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_grapheme_extend.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_grapheme_extend(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_grapheme_link`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_grapheme_link.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_grapheme_link(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_hex_digit`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_hex_digit.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_hex_digit(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_hyphen`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_hyphen.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_hyphen(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_id_continue`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_id_continue.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_id_continue(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_ideographic`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ideographic.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ideographic(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_id_start`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_id_start.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_id_start(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_ids_binary_operator`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ids_binary_operator.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ids_binary_operator(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_ids_trinary_operator`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_ids_trinary_operator.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_ids_trinary_operator(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_join_control`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_join_control.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_join_control(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_logical_order_exception`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_logical_order_exception.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_logical_order_exception(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_lowercase`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_lowercase.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_lowercase(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_math`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_math.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_math(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_noncharacter_code_point`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_noncharacter_code_point.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_noncharacter_code_point(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_nfc_inert`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfc_inert.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfc_inert(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_nfd_inert`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfd_inert.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfd_inert(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_nfkc_inert`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfkc_inert.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfkc_inert(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_nfkd_inert`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_nfkd_inert.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_nfkd_inert(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_pattern_syntax`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_pattern_syntax.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_pattern_syntax(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_pattern_white_space`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_pattern_white_space.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_pattern_white_space(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_prepended_concatenation_mark`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_prepended_concatenation_mark.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_prepended_concatenation_mark(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_print`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_print.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_print(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_quotation_mark`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_quotation_mark.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_quotation_mark(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_radical`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_radical.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_radical(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_regional_indicator`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_regional_indicator.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_regional_indicator(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_soft_dotted`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_soft_dotted.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_soft_dotted(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_segment_starter`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_segment_starter.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_segment_starter(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_case_sensitive`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_case_sensitive.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_case_sensitive(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_sentence_terminal`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_sentence_terminal.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_sentence_terminal(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_terminal_punctuation`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_terminal_punctuation.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_terminal_punctuation(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_unified_ideograph`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_unified_ideograph.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_unified_ideograph(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_uppercase`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_uppercase.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_uppercase(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_variation_selector`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_variation_selector.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_variation_selector(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_white_space`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_white_space.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_white_space(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_xdigit`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_xdigit.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_xdigit(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_xid_continue`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_xid_continue.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_xid_continue(const ICU4XDataProvider& provider);

  /**
   * 
   * 
   * See the [Rust documentation for `load_xid_start`](https://unicode-org.github.io/icu4x-docs/doc/icu/properties/sets/fn.load_xid_start.html) for more information.
   */
  static diplomat::result<ICU4XCodePointSetData, ICU4XError> load_xid_start(const ICU4XDataProvider& provider);
  inline const capi::ICU4XCodePointSetData* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XCodePointSetData* AsFFIMut() { return this->inner.get(); }
  inline ICU4XCodePointSetData(capi::ICU4XCodePointSetData* i) : inner(i) {}
  ICU4XCodePointSetData() = default;
  ICU4XCodePointSetData(ICU4XCodePointSetData&&) noexcept = default;
  ICU4XCodePointSetData& operator=(ICU4XCodePointSetData&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XCodePointSetData, ICU4XCodePointSetDataDeleter> inner;
};

#include "ICU4XDataProvider.hpp"

inline bool ICU4XCodePointSetData::contains(char32_t cp) const {
  return capi::ICU4XCodePointSetData_contains(this->inner.get(), cp);
}
inline bool ICU4XCodePointSetData::contains32(uint32_t cp) const {
  return capi::ICU4XCodePointSetData_contains32(this->inner.get(), cp);
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_for_general_category_group(const ICU4XDataProvider& provider, uint32_t group) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_for_general_category_group(provider.AsFFI(), group);
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_ascii_hex_digit(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_ascii_hex_digit(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_alnum(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_alnum(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_alphabetic(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_alphabetic(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_bidi_control(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_bidi_control(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_bidi_mirrored(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_bidi_mirrored(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_blank(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_blank(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_cased(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_cased(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_case_ignorable(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_case_ignorable(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_full_composition_exclusion(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_full_composition_exclusion(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_changes_when_casefolded(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_changes_when_casefolded(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_changes_when_casemapped(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_changes_when_casemapped(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_changes_when_nfkc_casefolded(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_changes_when_nfkc_casefolded(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_changes_when_lowercased(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_changes_when_lowercased(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_changes_when_titlecased(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_changes_when_titlecased(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_changes_when_uppercased(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_changes_when_uppercased(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_dash(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_dash(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_deprecated(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_deprecated(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_default_ignorable_code_point(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_default_ignorable_code_point(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_diacritic(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_diacritic(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_emoji_modifier_base(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_emoji_modifier_base(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_emoji_component(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_emoji_component(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_emoji_modifier(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_emoji_modifier(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_emoji(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_emoji(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_emoji_presentation(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_emoji_presentation(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_extender(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_extender(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_extended_pictographic(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_extended_pictographic(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_graph(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_graph(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_grapheme_base(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_grapheme_base(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_grapheme_extend(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_grapheme_extend(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_grapheme_link(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_grapheme_link(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_hex_digit(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_hex_digit(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_hyphen(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_hyphen(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_id_continue(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_id_continue(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_ideographic(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_ideographic(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_id_start(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_id_start(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_ids_binary_operator(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_ids_binary_operator(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_ids_trinary_operator(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_ids_trinary_operator(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_join_control(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_join_control(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_logical_order_exception(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_logical_order_exception(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_lowercase(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_lowercase(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_math(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_math(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_noncharacter_code_point(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_noncharacter_code_point(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_nfc_inert(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_nfc_inert(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_nfd_inert(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_nfd_inert(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_nfkc_inert(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_nfkc_inert(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_nfkd_inert(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_nfkd_inert(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_pattern_syntax(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_pattern_syntax(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_pattern_white_space(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_pattern_white_space(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_prepended_concatenation_mark(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_prepended_concatenation_mark(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_print(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_print(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_quotation_mark(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_quotation_mark(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_radical(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_radical(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_regional_indicator(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_regional_indicator(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_soft_dotted(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_soft_dotted(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_segment_starter(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_segment_starter(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_case_sensitive(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_case_sensitive(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_sentence_terminal(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_sentence_terminal(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_terminal_punctuation(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_terminal_punctuation(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_unified_ideograph(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_unified_ideograph(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_uppercase(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_uppercase(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_variation_selector(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_variation_selector(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_white_space(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_white_space(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_xdigit(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_xdigit(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_xid_continue(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_xid_continue(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XCodePointSetData, ICU4XError> ICU4XCodePointSetData::load_xid_start(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XCodePointSetData_load_xid_start(provider.AsFFI());
  diplomat::result<ICU4XCodePointSetData, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XCodePointSetData>(std::move(ICU4XCodePointSetData(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
#endif
