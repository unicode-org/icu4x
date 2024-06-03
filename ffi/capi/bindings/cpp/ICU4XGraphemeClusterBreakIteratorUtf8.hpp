#ifndef ICU4XGraphemeClusterBreakIteratorUtf8_HPP
#define ICU4XGraphemeClusterBreakIteratorUtf8_HPP

#include "ICU4XGraphemeClusterBreakIteratorUtf8.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XGraphemeClusterBreakIteratorUtf8.h"


inline int32_t ICU4XGraphemeClusterBreakIteratorUtf8::next() {
  auto result = capi::ICU4XGraphemeClusterBreakIteratorUtf8_next(this->AsFFI());
  return result;
}

inline const capi::ICU4XGraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XGraphemeClusterBreakIteratorUtf8*>(this);
}

inline capi::ICU4XGraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<capi::ICU4XGraphemeClusterBreakIteratorUtf8*>(this);
}

inline const ICU4XGraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterBreakIteratorUtf8::FromFFI(const capi::ICU4XGraphemeClusterBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const ICU4XGraphemeClusterBreakIteratorUtf8*>(ptr);
}

inline ICU4XGraphemeClusterBreakIteratorUtf8* ICU4XGraphemeClusterBreakIteratorUtf8::FromFFI(capi::ICU4XGraphemeClusterBreakIteratorUtf8* ptr) {
  return reinterpret_cast<ICU4XGraphemeClusterBreakIteratorUtf8*>(ptr);
}

inline void ICU4XGraphemeClusterBreakIteratorUtf8::operator delete(void* ptr) {
  capi::ICU4XGraphemeClusterBreakIteratorUtf8_destroy(reinterpret_cast<capi::ICU4XGraphemeClusterBreakIteratorUtf8*>(ptr));
}


#endif // ICU4XGraphemeClusterBreakIteratorUtf8_HPP
