#ifndef ICU4XCodePointMapData8_HPP
#define ICU4XCodePointMapData8_HPP

#include "ICU4XCodePointMapData8.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointMapData8.h"
#include "ICU4XCodePointRangeIterator.hpp"
#include "ICU4XCodePointSetData.hpp"
#include "ICU4XDataProvider.hpp"
#include "ICU4XError.hpp"


inline uint8_t ICU4XCodePointMapData8::get(char32_t cp) const {
  auto result = capi::ICU4XCodePointMapData8_get(this->AsFFI(),
    cp);
  return result;
}

inline uint8_t ICU4XCodePointMapData8::get32(uint32_t cp) const {
  auto result = capi::ICU4XCodePointMapData8_get32(this->AsFFI(),
    cp);
  return result;
}

inline uint32_t ICU4XCodePointMapData8::general_category_to_mask(uint8_t gc) {
  auto result = capi::ICU4XCodePointMapData8_general_category_to_mask(gc);
  return result;
}

inline std::unique_ptr<ICU4XCodePointRangeIterator> ICU4XCodePointMapData8::iter_ranges_for_value(uint8_t value) const {
  auto result = capi::ICU4XCodePointMapData8_iter_ranges_for_value(this->AsFFI(),
    value);
  return std::unique_ptr<ICU4XCodePointRangeIterator>(ICU4XCodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<ICU4XCodePointRangeIterator> ICU4XCodePointMapData8::iter_ranges_for_value_complemented(uint8_t value) const {
  auto result = capi::ICU4XCodePointMapData8_iter_ranges_for_value_complemented(this->AsFFI(),
    value);
  return std::unique_ptr<ICU4XCodePointRangeIterator>(ICU4XCodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<ICU4XCodePointRangeIterator> ICU4XCodePointMapData8::iter_ranges_for_mask(uint32_t mask) const {
  auto result = capi::ICU4XCodePointMapData8_iter_ranges_for_mask(this->AsFFI(),
    mask);
  return std::unique_ptr<ICU4XCodePointRangeIterator>(ICU4XCodePointRangeIterator::FromFFI(result));
}

inline std::unique_ptr<ICU4XCodePointSetData> ICU4XCodePointMapData8::get_set_for_value(uint8_t value) const {
  auto result = capi::ICU4XCodePointMapData8_get_set_for_value(this->AsFFI(),
    value);
  return std::unique_ptr<ICU4XCodePointSetData>(ICU4XCodePointSetData::FromFFI(result));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::load_general_category(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_load_general_category(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::load_bidi_class(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_load_bidi_class(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::load_east_asian_width(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_load_east_asian_width(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::load_hangul_syllable_type(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_load_hangul_syllable_type(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::load_indic_syllabic_category(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_load_indic_syllabic_category(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::load_line_break(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_load_line_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::try_grapheme_cluster_break(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_try_grapheme_cluster_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::load_word_break(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_load_word_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::load_sentence_break(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_load_sentence_break(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError> ICU4XCodePointMapData8::load_joining_type(const ICU4XDataProvider& provider) {
  auto result = capi::ICU4XCodePointMapData8_load_joining_type(provider.AsFFI());
  return result.is_ok ? diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Ok<std::unique_ptr<ICU4XCodePointMapData8>>(std::unique_ptr<ICU4XCodePointMapData8>(ICU4XCodePointMapData8::FromFFI(result.ok)))) : diplomat::result<std::unique_ptr<ICU4XCodePointMapData8>, ICU4XError>(diplomat::Err<ICU4XError>(ICU4XError::FromFFI(result.err)));
}

inline const capi::ICU4XCodePointMapData8* ICU4XCodePointMapData8::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCodePointMapData8*>(this);
}

inline capi::ICU4XCodePointMapData8* ICU4XCodePointMapData8::AsFFI() {
  return reinterpret_cast<capi::ICU4XCodePointMapData8*>(this);
}

inline const ICU4XCodePointMapData8* ICU4XCodePointMapData8::FromFFI(const capi::ICU4XCodePointMapData8* ptr) {
  return reinterpret_cast<const ICU4XCodePointMapData8*>(ptr);
}

inline ICU4XCodePointMapData8* ICU4XCodePointMapData8::FromFFI(capi::ICU4XCodePointMapData8* ptr) {
  return reinterpret_cast<ICU4XCodePointMapData8*>(ptr);
}

inline void ICU4XCodePointMapData8::operator delete(void* ptr) {
  capi::ICU4XCodePointMapData8_destroy(reinterpret_cast<capi::ICU4XCodePointMapData8*>(ptr));
}


#endif // ICU4XCodePointMapData8_HPP
