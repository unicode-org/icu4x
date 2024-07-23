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


namespace diplomat {
namespace capi {
    extern "C" {
    
    int32_t icu4x_LineBreakIteratorUtf16_next_mv1(diplomat::capi::LineBreakIteratorUtf16* self);
    
    
    void icu4x_LineBreakIteratorUtf16_destroy_mv1(LineBreakIteratorUtf16* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t LineBreakIteratorUtf16::next() {
  auto result = diplomat::capi::icu4x_LineBreakIteratorUtf16_next_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::LineBreakIteratorUtf16* LineBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LineBreakIteratorUtf16*>(this);
}

inline diplomat::capi::LineBreakIteratorUtf16* LineBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<diplomat::capi::LineBreakIteratorUtf16*>(this);
}

inline const LineBreakIteratorUtf16* LineBreakIteratorUtf16::FromFFI(const diplomat::capi::LineBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const LineBreakIteratorUtf16*>(ptr);
}

inline LineBreakIteratorUtf16* LineBreakIteratorUtf16::FromFFI(diplomat::capi::LineBreakIteratorUtf16* ptr) {
  return reinterpret_cast<LineBreakIteratorUtf16*>(ptr);
}

inline void LineBreakIteratorUtf16::operator delete(void* ptr) {
  diplomat::capi::icu4x_LineBreakIteratorUtf16_destroy_mv1(reinterpret_cast<diplomat::capi::LineBreakIteratorUtf16*>(ptr));
}


#endif // LineBreakIteratorUtf16_HPP
