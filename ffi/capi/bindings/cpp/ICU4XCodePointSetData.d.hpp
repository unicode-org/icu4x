#ifndef ICU4XCodePointSetData_D_HPP
#define ICU4XCodePointSetData_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointSetData.d.h"
#include "ICU4XDataError.d.hpp"
#include "ICU4XError.d.hpp"

class ICU4XCodePointRangeIterator;
class ICU4XDataProvider;
class ICU4XDataError;
class ICU4XError;


class ICU4XCodePointSetData {
public:

  inline bool contains(char32_t cp) const;

  inline bool contains32(uint32_t cp) const;

  inline std::unique_ptr<ICU4XCodePointRangeIterator> iter_ranges() const;

  inline std::unique_ptr<ICU4XCodePointRangeIterator> iter_ranges_complemented() const;

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_for_general_category_group(const ICU4XDataProvider& provider, uint32_t group);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_ascii_hex_digit(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_alnum(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_alphabetic(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_bidi_control(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_bidi_mirrored(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_blank(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_cased(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_case_ignorable(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_full_composition_exclusion(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_changes_when_casefolded(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_changes_when_casemapped(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_changes_when_nfkc_casefolded(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_changes_when_lowercased(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_changes_when_titlecased(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_changes_when_uppercased(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_dash(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_deprecated(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_default_ignorable_code_point(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_diacritic(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_emoji_modifier_base(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_emoji_component(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_emoji_modifier(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_emoji(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_emoji_presentation(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_extender(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_extended_pictographic(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_graph(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_grapheme_base(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_grapheme_extend(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_grapheme_link(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_hex_digit(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_hyphen(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_id_continue(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_ideographic(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_id_start(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_ids_binary_operator(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_ids_trinary_operator(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_join_control(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_logical_order_exception(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_lowercase(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_math(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_noncharacter_code_point(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_nfc_inert(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_nfd_inert(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_nfkc_inert(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_nfkd_inert(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_pattern_syntax(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_pattern_white_space(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_prepended_concatenation_mark(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_print(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_quotation_mark(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_radical(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_regional_indicator(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_soft_dotted(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_segment_starter(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_case_sensitive(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_sentence_terminal(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_terminal_punctuation(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_unified_ideograph(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_uppercase(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_variation_selector(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_white_space(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_xdigit(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_xid_continue(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XDataError> load_xid_start(const ICU4XDataProvider& provider);

  inline static diplomat::result<diplomat::result<std::unique_ptr<ICU4XCodePointSetData>, ICU4XError>, diplomat::Utf8Error> load_for_ecma262(const ICU4XDataProvider& provider, std::string_view property_name);

  inline const capi::ICU4XCodePointSetData* AsFFI() const;
  inline capi::ICU4XCodePointSetData* AsFFI();
  inline static const ICU4XCodePointSetData* FromFFI(const capi::ICU4XCodePointSetData* ptr);
  inline static ICU4XCodePointSetData* FromFFI(capi::ICU4XCodePointSetData* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCodePointSetData() = delete;
  ICU4XCodePointSetData(const ICU4XCodePointSetData&) = delete;
  ICU4XCodePointSetData(ICU4XCodePointSetData&&) noexcept = delete;
  ICU4XCodePointSetData operator=(const ICU4XCodePointSetData&) = delete;
  ICU4XCodePointSetData operator=(ICU4XCodePointSetData&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCodePointSetData_D_HPP
