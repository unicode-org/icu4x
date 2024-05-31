#ifndef ICU4XSentenceBreakIteratorUtf8_HPP
#define ICU4XSentenceBreakIteratorUtf8_HPP

#include "ICU4XSentenceBreakIteratorUtf8.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XSentenceBreakIteratorUtf8.h"


inline int32_t ICU4XSentenceBreakIteratorUtf8::next() {
  auto result = capi::ICU4XSentenceBreakIteratorUtf8_next(this->AsFFI());
  return result;
}

inline const capi::ICU4XSentenceBreakIteratorUtf8* ICU4XSentenceBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XSentenceBreakIteratorUtf8*>(this);
}

inline capi::ICU4XSentenceBreakIteratorUtf8* ICU4XSentenceBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<capi::ICU4XSentenceBreakIteratorUtf8*>(this);
}

inline const ICU4XSentenceBreakIteratorUtf8* ICU4XSentenceBreakIteratorUtf8::FromFFI(const capi::ICU4XSentenceBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const ICU4XSentenceBreakIteratorUtf8*>(ptr);
}

inline ICU4XSentenceBreakIteratorUtf8* ICU4XSentenceBreakIteratorUtf8::FromFFI(capi::ICU4XSentenceBreakIteratorUtf8* ptr) {
  return reinterpret_cast<ICU4XSentenceBreakIteratorUtf8*>(ptr);
}

inline void ICU4XSentenceBreakIteratorUtf8::operator delete(void* ptr) {
  capi::ICU4XSentenceBreakIteratorUtf8_destroy(reinterpret_cast<capi::ICU4XSentenceBreakIteratorUtf8*>(ptr));
}


#endif // ICU4XSentenceBreakIteratorUtf8_HPP
