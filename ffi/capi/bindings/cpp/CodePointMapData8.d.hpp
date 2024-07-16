#ifndef CodePointMapData8_D_HPP
#define CodePointMapData8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct CodePointRangeIterator; }
class CodePointRangeIterator;
namespace diplomat::capi { struct CodePointSetData; }
class CodePointSetData;
namespace diplomat::capi { struct DataProvider; }
class DataProvider;
class DataError;


namespace diplomat {
namespace capi {
    struct CodePointMapData8;
} // namespace capi
} // namespace

class CodePointMapData8 {
public:

  inline uint8_t get(char32_t cp) const;

  inline uint8_t get32(uint32_t cp) const;

  inline static uint32_t general_category_to_mask(uint8_t gc);

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_value(uint8_t value) const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_value_complemented(uint8_t value) const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_mask(uint32_t mask) const;

  inline std::unique_ptr<CodePointSetData> get_set_for_value(uint8_t value) const;

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> load_general_category(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> load_bidi_class(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> load_east_asian_width(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> load_hangul_syllable_type(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> load_indic_syllabic_category(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> load_line_break(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> try_grapheme_cluster_break(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> load_word_break(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> load_sentence_break(const DataProvider& provider);

  inline static diplomat::result<std::unique_ptr<CodePointMapData8>, DataError> load_joining_type(const DataProvider& provider);

  inline const diplomat::capi::CodePointMapData8* AsFFI() const;
  inline diplomat::capi::CodePointMapData8* AsFFI();
  inline static const CodePointMapData8* FromFFI(const diplomat::capi::CodePointMapData8* ptr);
  inline static CodePointMapData8* FromFFI(diplomat::capi::CodePointMapData8* ptr);
  inline static void operator delete(void* ptr);
private:
  CodePointMapData8() = delete;
  CodePointMapData8(const CodePointMapData8&) = delete;
  CodePointMapData8(CodePointMapData8&&) noexcept = delete;
  CodePointMapData8 operator=(const CodePointMapData8&) = delete;
  CodePointMapData8 operator=(CodePointMapData8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // CodePointMapData8_D_HPP
