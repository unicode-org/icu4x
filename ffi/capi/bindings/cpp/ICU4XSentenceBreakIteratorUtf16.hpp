#ifndef ICU4XSentenceBreakIteratorUtf16_HPP
#define ICU4XSentenceBreakIteratorUtf16_HPP

#include "ICU4XSentenceBreakIteratorUtf16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XSentenceBreakIteratorUtf16.h"


inline int32_t ICU4XSentenceBreakIteratorUtf16::next() {
  auto result = capi::ICU4XSentenceBreakIteratorUtf16_next(this->AsFFI());
  return result;
}

inline const capi::ICU4XSentenceBreakIteratorUtf16* ICU4XSentenceBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XSentenceBreakIteratorUtf16*>(this);
}

inline capi::ICU4XSentenceBreakIteratorUtf16* ICU4XSentenceBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<capi::ICU4XSentenceBreakIteratorUtf16*>(this);
}

inline const ICU4XSentenceBreakIteratorUtf16* ICU4XSentenceBreakIteratorUtf16::FromFFI(const capi::ICU4XSentenceBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const ICU4XSentenceBreakIteratorUtf16*>(ptr);
}

inline ICU4XSentenceBreakIteratorUtf16* ICU4XSentenceBreakIteratorUtf16::FromFFI(capi::ICU4XSentenceBreakIteratorUtf16* ptr) {
  return reinterpret_cast<ICU4XSentenceBreakIteratorUtf16*>(ptr);
}

inline void ICU4XSentenceBreakIteratorUtf16::operator delete(void* ptr) {
  capi::ICU4XSentenceBreakIteratorUtf16_destroy(reinterpret_cast<capi::ICU4XSentenceBreakIteratorUtf16*>(ptr));
}


#endif // ICU4XSentenceBreakIteratorUtf16_HPP
