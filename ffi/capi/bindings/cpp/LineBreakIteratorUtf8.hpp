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


namespace capi {
    extern "C" {
    
    int32_t ICU4XLineBreakIteratorUtf8_next(LineBreakIteratorUtf8* self);
    
    
    void ICU4XLineBreakIteratorUtf8_destroy(LineBreakIteratorUtf8* self);
    
    } // extern "C"
}

inline int32_t LineBreakIteratorUtf8::next() {
  auto result = capi::ICU4XLineBreakIteratorUtf8_next(this->AsFFI());
  return result;
}

inline const capi::LineBreakIteratorUtf8* LineBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const capi::LineBreakIteratorUtf8*>(this);
}

inline capi::LineBreakIteratorUtf8* LineBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<capi::LineBreakIteratorUtf8*>(this);
}

inline const LineBreakIteratorUtf8* LineBreakIteratorUtf8::FromFFI(const capi::LineBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const LineBreakIteratorUtf8*>(ptr);
}

inline LineBreakIteratorUtf8* LineBreakIteratorUtf8::FromFFI(capi::LineBreakIteratorUtf8* ptr) {
  return reinterpret_cast<LineBreakIteratorUtf8*>(ptr);
}

inline void LineBreakIteratorUtf8::operator delete(void* ptr) {
  capi::ICU4XLineBreakIteratorUtf8_destroy(reinterpret_cast<capi::LineBreakIteratorUtf8*>(ptr));
}


#endif // LineBreakIteratorUtf8_HPP
