#ifndef ICU4XCodePointMapData8_D_HPP
#define ICU4XCodePointMapData8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointMapData8.d.h"
#include "ICU4XError.d.hpp"

class CodePointRangeIterator;
class ICU4XCodePointSetData;
class ICU4XDataProvider;
class ICU4XError;


class ICU4XCodePointMapData8 {
public:

  inline uint8_t get(char32_t cp) const;

  inline uint8_t get32(uint32_t cp) const;

  inline static uint32_t general_category_to_mask(uint8_t gc);

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_value(uint8_t value) const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_value_complemented(uint8_t value) const;

  inline std::unique_ptr<CodePointRangeIterator> iter_ranges_for_mask(uint32_t mask) const;

  inline std::unique_ptr<ICU4XCodePointSetData> get_set_for_value(uint8_t value) const;

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> load_general_category(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> load_bidi_class(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> load_east_asian_width(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> load_hangul_syllable_type(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> load_indic_syllabic_category(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> load_line_break(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> try_grapheme_cluster_break(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> load_word_break(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> load_sentence_break(const ICU4XDataProvider& provider);

  inline static diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> load_joining_type(const ICU4XDataProvider& provider);

  inline const capi::ICU4XCodePointMapData8* AsFFI() const;
  inline capi::ICU4XCodePointMapData8* AsFFI();
  inline static const ICU4XCodePointMapData8* FromFFI(const capi::ICU4XCodePointMapData8* ptr);
  inline static ICU4XCodePointMapData8* FromFFI(capi::ICU4XCodePointMapData8* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XCodePointMapData8() = delete;
  ICU4XCodePointMapData8(const ICU4XCodePointMapData8&) = delete;
  ICU4XCodePointMapData8(ICU4XCodePointMapData8&&) noexcept = delete;
  ICU4XCodePointMapData8 operator=(const ICU4XCodePointMapData8&) = delete;
  ICU4XCodePointMapData8 operator=(ICU4XCodePointMapData8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XCodePointMapData8_D_HPP
