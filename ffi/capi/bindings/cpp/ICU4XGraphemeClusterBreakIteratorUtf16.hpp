#ifndef ICU4XGraphemeClusterBreakIteratorUtf16_HPP
#define ICU4XGraphemeClusterBreakIteratorUtf16_HPP

#include "ICU4XGraphemeClusterBreakIteratorUtf16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    int32_t ICU4XGraphemeClusterBreakIteratorUtf16_next(ICU4XGraphemeClusterBreakIteratorUtf16* self);
    
    
    void ICU4XGraphemeClusterBreakIteratorUtf16_destroy(ICU4XGraphemeClusterBreakIteratorUtf16* self);
    
    } // extern "C"
}

inline int32_t ICU4XGraphemeClusterBreakIteratorUtf16::next() {
  auto result = capi::ICU4XGraphemeClusterBreakIteratorUtf16_next(this->AsFFI());
  return result;
}

inline const capi::ICU4XGraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XGraphemeClusterBreakIteratorUtf16*>(this);
}

inline capi::ICU4XGraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<capi::ICU4XGraphemeClusterBreakIteratorUtf16*>(this);
}

inline const ICU4XGraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterBreakIteratorUtf16::FromFFI(const capi::ICU4XGraphemeClusterBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const ICU4XGraphemeClusterBreakIteratorUtf16*>(ptr);
}

inline ICU4XGraphemeClusterBreakIteratorUtf16* ICU4XGraphemeClusterBreakIteratorUtf16::FromFFI(capi::ICU4XGraphemeClusterBreakIteratorUtf16* ptr) {
  return reinterpret_cast<ICU4XGraphemeClusterBreakIteratorUtf16*>(ptr);
}

inline void ICU4XGraphemeClusterBreakIteratorUtf16::operator delete(void* ptr) {
  capi::ICU4XGraphemeClusterBreakIteratorUtf16_destroy(reinterpret_cast<capi::ICU4XGraphemeClusterBreakIteratorUtf16*>(ptr));
}


#endif // ICU4XGraphemeClusterBreakIteratorUtf16_HPP
