#ifndef ICU4XLineBreakIteratorLatin1_HPP
#define ICU4XLineBreakIteratorLatin1_HPP

#include "ICU4XLineBreakIteratorLatin1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLineBreakIteratorLatin1.h"


inline int32_t ICU4XLineBreakIteratorLatin1::next() {
  auto result = capi::ICU4XLineBreakIteratorLatin1_next(this->AsFFI());
  return result;
}

inline const capi::ICU4XLineBreakIteratorLatin1* ICU4XLineBreakIteratorLatin1::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLineBreakIteratorLatin1*>(this);
}

inline capi::ICU4XLineBreakIteratorLatin1* ICU4XLineBreakIteratorLatin1::AsFFI() {
  return reinterpret_cast<capi::ICU4XLineBreakIteratorLatin1*>(this);
}

inline const ICU4XLineBreakIteratorLatin1* ICU4XLineBreakIteratorLatin1::FromFFI(const capi::ICU4XLineBreakIteratorLatin1* ptr) {
  return reinterpret_cast<const ICU4XLineBreakIteratorLatin1*>(ptr);
}

inline ICU4XLineBreakIteratorLatin1* ICU4XLineBreakIteratorLatin1::FromFFI(capi::ICU4XLineBreakIteratorLatin1* ptr) {
  return reinterpret_cast<ICU4XLineBreakIteratorLatin1*>(ptr);
}

inline void ICU4XLineBreakIteratorLatin1::operator delete(void* ptr) {
  capi::ICU4XLineBreakIteratorLatin1_destroy(reinterpret_cast<capi::ICU4XLineBreakIteratorLatin1*>(ptr));
}


#endif // ICU4XLineBreakIteratorLatin1_HPP
