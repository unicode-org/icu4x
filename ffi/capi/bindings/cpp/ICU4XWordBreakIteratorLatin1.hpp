#ifndef ICU4XWordBreakIteratorLatin1_HPP
#define ICU4XWordBreakIteratorLatin1_HPP

#include "ICU4XWordBreakIteratorLatin1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XSegmenterWordType.hpp"
#include "ICU4XWordBreakIteratorLatin1.h"


inline int32_t ICU4XWordBreakIteratorLatin1::next() {
  auto result = capi::ICU4XWordBreakIteratorLatin1_next(this->AsFFI());
  return result;
}

inline ICU4XSegmenterWordType ICU4XWordBreakIteratorLatin1::word_type() const {
  auto result = capi::ICU4XWordBreakIteratorLatin1_word_type(this->AsFFI());
  return ICU4XSegmenterWordType::FromFFI(result);
}

inline bool ICU4XWordBreakIteratorLatin1::is_word_like() const {
  auto result = capi::ICU4XWordBreakIteratorLatin1_is_word_like(this->AsFFI());
  return result;
}

inline const capi::ICU4XWordBreakIteratorLatin1* ICU4XWordBreakIteratorLatin1::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XWordBreakIteratorLatin1*>(this);
}

inline capi::ICU4XWordBreakIteratorLatin1* ICU4XWordBreakIteratorLatin1::AsFFI() {
  return reinterpret_cast<capi::ICU4XWordBreakIteratorLatin1*>(this);
}

inline const ICU4XWordBreakIteratorLatin1* ICU4XWordBreakIteratorLatin1::FromFFI(const capi::ICU4XWordBreakIteratorLatin1* ptr) {
  return reinterpret_cast<const ICU4XWordBreakIteratorLatin1*>(ptr);
}

inline ICU4XWordBreakIteratorLatin1* ICU4XWordBreakIteratorLatin1::FromFFI(capi::ICU4XWordBreakIteratorLatin1* ptr) {
  return reinterpret_cast<ICU4XWordBreakIteratorLatin1*>(ptr);
}

inline void ICU4XWordBreakIteratorLatin1::operator delete(void* ptr) {
  capi::ICU4XWordBreakIteratorLatin1_destroy(reinterpret_cast<capi::ICU4XWordBreakIteratorLatin1*>(ptr));
}


#endif // ICU4XWordBreakIteratorLatin1_HPP
