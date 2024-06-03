#ifndef ICU4XWordBreakIteratorUtf16_HPP
#define ICU4XWordBreakIteratorUtf16_HPP

#include "ICU4XWordBreakIteratorUtf16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XSegmenterWordType.hpp"
#include "ICU4XWordBreakIteratorUtf16.h"


inline int32_t ICU4XWordBreakIteratorUtf16::next() {
  auto result = capi::ICU4XWordBreakIteratorUtf16_next(this->AsFFI());
  return result;
}

inline ICU4XSegmenterWordType ICU4XWordBreakIteratorUtf16::word_type() const {
  auto result = capi::ICU4XWordBreakIteratorUtf16_word_type(this->AsFFI());
  return ICU4XSegmenterWordType::FromFFI(result);
}

inline bool ICU4XWordBreakIteratorUtf16::is_word_like() const {
  auto result = capi::ICU4XWordBreakIteratorUtf16_is_word_like(this->AsFFI());
  return result;
}

inline const capi::ICU4XWordBreakIteratorUtf16* ICU4XWordBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XWordBreakIteratorUtf16*>(this);
}

inline capi::ICU4XWordBreakIteratorUtf16* ICU4XWordBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<capi::ICU4XWordBreakIteratorUtf16*>(this);
}

inline const ICU4XWordBreakIteratorUtf16* ICU4XWordBreakIteratorUtf16::FromFFI(const capi::ICU4XWordBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const ICU4XWordBreakIteratorUtf16*>(ptr);
}

inline ICU4XWordBreakIteratorUtf16* ICU4XWordBreakIteratorUtf16::FromFFI(capi::ICU4XWordBreakIteratorUtf16* ptr) {
  return reinterpret_cast<ICU4XWordBreakIteratorUtf16*>(ptr);
}

inline void ICU4XWordBreakIteratorUtf16::operator delete(void* ptr) {
  capi::ICU4XWordBreakIteratorUtf16_destroy(reinterpret_cast<capi::ICU4XWordBreakIteratorUtf16*>(ptr));
}


#endif // ICU4XWordBreakIteratorUtf16_HPP
