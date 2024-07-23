#ifndef LineBreakIteratorUtf8_HPP
#define LineBreakIteratorUtf8_HPP

#include "LineBreakIteratorUtf8.d.hpp"

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
    
    int32_t icu4x_LineBreakIteratorUtf8_next_mv1(diplomat::capi::LineBreakIteratorUtf8* self);
    
    
    void icu4x_LineBreakIteratorUtf8_destroy_mv1(LineBreakIteratorUtf8* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t LineBreakIteratorUtf8::next() {
  auto result = diplomat::capi::icu4x_LineBreakIteratorUtf8_next_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::LineBreakIteratorUtf8* LineBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LineBreakIteratorUtf8*>(this);
}

inline diplomat::capi::LineBreakIteratorUtf8* LineBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<diplomat::capi::LineBreakIteratorUtf8*>(this);
}

inline const LineBreakIteratorUtf8* LineBreakIteratorUtf8::FromFFI(const diplomat::capi::LineBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const LineBreakIteratorUtf8*>(ptr);
}

inline LineBreakIteratorUtf8* LineBreakIteratorUtf8::FromFFI(diplomat::capi::LineBreakIteratorUtf8* ptr) {
  return reinterpret_cast<LineBreakIteratorUtf8*>(ptr);
}

inline void LineBreakIteratorUtf8::operator delete(void* ptr) {
  diplomat::capi::icu4x_LineBreakIteratorUtf8_destroy_mv1(reinterpret_cast<diplomat::capi::LineBreakIteratorUtf8*>(ptr));
}


#endif // LineBreakIteratorUtf8_HPP
