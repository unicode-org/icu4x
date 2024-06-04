#ifndef ICU4XGraphemeClusterBreakIteratorLatin1_HPP
#define ICU4XGraphemeClusterBreakIteratorLatin1_HPP

#include "ICU4XGraphemeClusterBreakIteratorLatin1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XGraphemeClusterBreakIteratorLatin1.h"


inline int32_t ICU4XGraphemeClusterBreakIteratorLatin1::next() {
  auto result = capi::ICU4XGraphemeClusterBreakIteratorLatin1_next(this->AsFFI());
  return result;
}

inline const capi::ICU4XGraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterBreakIteratorLatin1::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XGraphemeClusterBreakIteratorLatin1*>(this);
}

inline capi::ICU4XGraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterBreakIteratorLatin1::AsFFI() {
  return reinterpret_cast<capi::ICU4XGraphemeClusterBreakIteratorLatin1*>(this);
}

inline const ICU4XGraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterBreakIteratorLatin1::FromFFI(const capi::ICU4XGraphemeClusterBreakIteratorLatin1* ptr) {
  return reinterpret_cast<const ICU4XGraphemeClusterBreakIteratorLatin1*>(ptr);
}

inline ICU4XGraphemeClusterBreakIteratorLatin1* ICU4XGraphemeClusterBreakIteratorLatin1::FromFFI(capi::ICU4XGraphemeClusterBreakIteratorLatin1* ptr) {
  return reinterpret_cast<ICU4XGraphemeClusterBreakIteratorLatin1*>(ptr);
}

inline void ICU4XGraphemeClusterBreakIteratorLatin1::operator delete(void* ptr) {
  capi::ICU4XGraphemeClusterBreakIteratorLatin1_destroy(reinterpret_cast<capi::ICU4XGraphemeClusterBreakIteratorLatin1*>(ptr));
}


#endif // ICU4XGraphemeClusterBreakIteratorLatin1_HPP
