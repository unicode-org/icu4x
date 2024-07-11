#ifndef LineBreakIteratorUtf16_HPP
#define LineBreakIteratorUtf16_HPP

#include "LineBreakIteratorUtf16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    int32_t ICU4XLineBreakIteratorUtf16_next(LineBreakIteratorUtf16* self);
    
    
    void ICU4XLineBreakIteratorUtf16_destroy(LineBreakIteratorUtf16* self);
    
    } // extern "C"
}

inline int32_t LineBreakIteratorUtf16::next() {
  auto result = capi::ICU4XLineBreakIteratorUtf16_next(this->AsFFI());
  return result;
}

inline const capi::LineBreakIteratorUtf16* LineBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const capi::LineBreakIteratorUtf16*>(this);
}

inline capi::LineBreakIteratorUtf16* LineBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<capi::LineBreakIteratorUtf16*>(this);
}

inline const LineBreakIteratorUtf16* LineBreakIteratorUtf16::FromFFI(const capi::LineBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const LineBreakIteratorUtf16*>(ptr);
}

inline LineBreakIteratorUtf16* LineBreakIteratorUtf16::FromFFI(capi::LineBreakIteratorUtf16* ptr) {
  return reinterpret_cast<LineBreakIteratorUtf16*>(ptr);
}

inline void LineBreakIteratorUtf16::operator delete(void* ptr) {
  capi::ICU4XLineBreakIteratorUtf16_destroy(reinterpret_cast<capi::LineBreakIteratorUtf16*>(ptr));
}


#endif // LineBreakIteratorUtf16_HPP
