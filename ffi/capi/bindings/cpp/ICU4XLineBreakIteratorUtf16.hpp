#ifndef ICU4XLineBreakIteratorUtf16_HPP
#define ICU4XLineBreakIteratorUtf16_HPP

#include "ICU4XLineBreakIteratorUtf16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    int32_t ICU4XLineBreakIteratorUtf16_next(ICU4XLineBreakIteratorUtf16* self);
    
    
    void ICU4XLineBreakIteratorUtf16_destroy(ICU4XLineBreakIteratorUtf16* self);
    
    } // extern "C"
}

inline int32_t ICU4XLineBreakIteratorUtf16::next() {
  auto result = capi::ICU4XLineBreakIteratorUtf16_next(this->AsFFI());
  return result;
}

inline const capi::ICU4XLineBreakIteratorUtf16* ICU4XLineBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XLineBreakIteratorUtf16*>(this);
}

inline capi::ICU4XLineBreakIteratorUtf16* ICU4XLineBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<capi::ICU4XLineBreakIteratorUtf16*>(this);
}

inline const ICU4XLineBreakIteratorUtf16* ICU4XLineBreakIteratorUtf16::FromFFI(const capi::ICU4XLineBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const ICU4XLineBreakIteratorUtf16*>(ptr);
}

inline ICU4XLineBreakIteratorUtf16* ICU4XLineBreakIteratorUtf16::FromFFI(capi::ICU4XLineBreakIteratorUtf16* ptr) {
  return reinterpret_cast<ICU4XLineBreakIteratorUtf16*>(ptr);
}

inline void ICU4XLineBreakIteratorUtf16::operator delete(void* ptr) {
  capi::ICU4XLineBreakIteratorUtf16_destroy(reinterpret_cast<capi::ICU4XLineBreakIteratorUtf16*>(ptr));
}


#endif // ICU4XLineBreakIteratorUtf16_HPP
