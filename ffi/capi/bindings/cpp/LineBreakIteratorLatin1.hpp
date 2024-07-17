#ifndef LineBreakIteratorLatin1_HPP
#define LineBreakIteratorLatin1_HPP

#include "LineBreakIteratorLatin1.d.hpp"

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
    
    int32_t icu4x_LineBreakIteratorLatin1_next_mv1(diplomat::capi::LineBreakIteratorLatin1* self);
    
    
    void icu4x_LineBreakIteratorLatin1_destroy_mv1(LineBreakIteratorLatin1* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t LineBreakIteratorLatin1::next() {
  auto result = diplomat::capi::icu4x_LineBreakIteratorLatin1_next_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::LineBreakIteratorLatin1* LineBreakIteratorLatin1::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::LineBreakIteratorLatin1*>(this);
}

inline diplomat::capi::LineBreakIteratorLatin1* LineBreakIteratorLatin1::AsFFI() {
  return reinterpret_cast<diplomat::capi::LineBreakIteratorLatin1*>(this);
}

inline const LineBreakIteratorLatin1* LineBreakIteratorLatin1::FromFFI(const diplomat::capi::LineBreakIteratorLatin1* ptr) {
  return reinterpret_cast<const LineBreakIteratorLatin1*>(ptr);
}

inline LineBreakIteratorLatin1* LineBreakIteratorLatin1::FromFFI(diplomat::capi::LineBreakIteratorLatin1* ptr) {
  return reinterpret_cast<LineBreakIteratorLatin1*>(ptr);
}

inline void LineBreakIteratorLatin1::operator delete(void* ptr) {
  diplomat::capi::icu4x_LineBreakIteratorLatin1_destroy_mv1(reinterpret_cast<diplomat::capi::LineBreakIteratorLatin1*>(ptr));
}


#endif // LineBreakIteratorLatin1_HPP
