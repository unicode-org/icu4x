#ifndef CodePointSetData_D_HPP
#define CodePointSetData_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct CodePointRangeIterator; }
class CodePointRangeIterator;
namespace diplomat::capi { struct DataProvider; }
class DataProvider;
class DataError;
class Error;


namespace diplomat {
namespace capi {
    struct CodePointSetData;
} // namespace capi
} // namespace

class CodePointSetData {
public:

  inline bool contains(char32_t cp) const;

  inline bool contains32(uint32_t cp) const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges() const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_complemented() const;

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_for_general_category_group(const DataProvider& provider, uint32_t group);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_ascii_hex_digit(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_alnum(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_alphabetic(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_bidi_control(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_bidi_mirrored(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_blank(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_cased(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_case_ignorable(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_full_composition_exclusion(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_changes_when_casefolded(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_changes_when_casemapped(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_changes_when_nfkc_casefolded(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_changes_when_lowercased(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_changes_when_titlecased(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_changes_when_uppercased(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_dash(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_deprecated(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_default_ignorable_code_point(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_diacritic(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_emoji_modifier_base(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_emoji_component(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_emoji_modifier(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_emoji(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_emoji_presentation(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_extender(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_extended_pictographic(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_graph(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_grapheme_base(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_grapheme_extend(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_grapheme_link(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_hex_digit(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_hyphen(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_id_continue(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_ideographic(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_id_start(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_ids_binary_operator(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_ids_trinary_operator(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_join_control(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_logical_order_exception(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_lowercase(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_math(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_noncharacter_code_point(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_nfc_inert(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_nfd_inert(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_nfkc_inert(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_nfkd_inert(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_pattern_syntax(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_pattern_white_space(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_prepended_concatenation_mark(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_print(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_quotation_mark(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_radical(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_regional_indicator(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_soft_dotted(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_segment_starter(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_case_sensitive(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_sentence_terminal(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_terminal_punctuation(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_unified_ideograph(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_uppercase(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_variation_selector(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_white_space(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_xdigit(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_xid_continue(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointSetData>, DataError> load_xid_start(const DataProvider& provider);

  inline static diplomat::result<diplomat::result<std::unique_ptr<CodePointSetData>, Error>, diplomat::Utf8Error> load_for_ecma262(const DataProvider& provider, std::string_view property_name);

  inline const diplomat::capi::CodePointSetData* AsFFI() const;
  inline diplomat::capi::CodePointSetData* AsFFI();
  inline static const CodePointSetData* FromFFI(const diplomat::capi::CodePointSetData* ptr);
  inline static CodePointSetData* FromFFI(diplomat::capi::CodePointSetData* ptr);
  inline static void operator delete(void* ptr);
private:
  CodePointSetData() = delete;
  CodePointSetData(const CodePointSetData&) = delete;
  CodePointSetData(CodePointSetData&&) noexcept = delete;
  CodePointSetData operator=(const CodePointSetData&) = delete;
  CodePointSetData operator=(CodePointSetData&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CodePointSetData_D_HPP
