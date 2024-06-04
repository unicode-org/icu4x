#ifndef ICU4XLineBreakIteratorUtf8_HPP
#define ICU4XLineBreakIteratorUtf8_HPP

#include "ICU4XLineBreakIteratorUtf8.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLineBreakIteratorUtf8.h"


inline int32_t ICU4XLineBreakIteratorUtf8::next() {
  auto result = capi::ICU4XLineBreakIteratorUtf8_next(this->AsFFI());
  return result;
}

inline const capi::ICU4XLineBreakIteratorUtf8* ICU4XLineBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLineBreakIteratorUtf8*>(this);
}

inline capi::ICU4XLineBreakIteratorUtf8* ICU4XLineBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<capi::ICU4XLineBreakIteratorUtf8*>(this);
}

inline const ICU4XLineBreakIteratorUtf8* ICU4XLineBreakIteratorUtf8::FromFFI(const capi::ICU4XLineBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const ICU4XLineBreakIteratorUtf8*>(ptr);
}

inline ICU4XLineBreakIteratorUtf8* ICU4XLineBreakIteratorUtf8::FromFFI(capi::ICU4XLineBreakIteratorUtf8* ptr) {
  return reinterpret_cast<ICU4XLineBreakIteratorUtf8*>(ptr);
}

inline void ICU4XLineBreakIteratorUtf8::operator delete(void* ptr) {
  capi::ICU4XLineBreakIteratorUtf8_destroy(reinterpret_cast<capi::ICU4XLineBreakIteratorUtf8*>(ptr));
}


#endif // ICU4XLineBreakIteratorUtf8_HPP
