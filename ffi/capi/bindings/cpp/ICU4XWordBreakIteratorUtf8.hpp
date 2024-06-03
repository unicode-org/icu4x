#ifndef ICU4XWordBreakIteratorUtf8_HPP
#define ICU4XWordBreakIteratorUtf8_HPP

#include "ICU4XWordBreakIteratorUtf8.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XSegmenterWordType.hpp"
#include "ICU4XWordBreakIteratorUtf8.h"


inline int32_t ICU4XWordBreakIteratorUtf8::next() {
  auto result = capi::ICU4XWordBreakIteratorUtf8_next(this->AsFFI());
  return result;
}

inline ICU4XSegmenterWordType ICU4XWordBreakIteratorUtf8::word_type() const {
  auto result = capi::ICU4XWordBreakIteratorUtf8_word_type(this->AsFFI());
  return ICU4XSegmenterWordType::FromFFI(result);
}

inline bool ICU4XWordBreakIteratorUtf8::is_word_like() const {
  auto result = capi::ICU4XWordBreakIteratorUtf8_is_word_like(this->AsFFI());
  return result;
}

inline const capi::ICU4XWordBreakIteratorUtf8* ICU4XWordBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XWordBreakIteratorUtf8*>(this);
}

inline capi::ICU4XWordBreakIteratorUtf8* ICU4XWordBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<capi::ICU4XWordBreakIteratorUtf8*>(this);
}

inline const ICU4XWordBreakIteratorUtf8* ICU4XWordBreakIteratorUtf8::FromFFI(const capi::ICU4XWordBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const ICU4XWordBreakIteratorUtf8*>(ptr);
}

inline ICU4XWordBreakIteratorUtf8* ICU4XWordBreakIteratorUtf8::FromFFI(capi::ICU4XWordBreakIteratorUtf8* ptr) {
  return reinterpret_cast<ICU4XWordBreakIteratorUtf8*>(ptr);
}

inline void ICU4XWordBreakIteratorUtf8::operator delete(void* ptr) {
  capi::ICU4XWordBreakIteratorUtf8_destroy(reinterpret_cast<capi::ICU4XWordBreakIteratorUtf8*>(ptr));
}


#endif // ICU4XWordBreakIteratorUtf8_HPP
