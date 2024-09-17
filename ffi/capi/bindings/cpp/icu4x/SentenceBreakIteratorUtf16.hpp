#ifndef icu4x_SentenceBreakIteratorUtf16_HPP
#define icu4x_SentenceBreakIteratorUtf16_HPP

#include "SentenceBreakIteratorUtf16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    int32_t icu4x_SentenceBreakIteratorUtf16_next_mv1(icu4x::capi::SentenceBreakIteratorUtf16* self);
    
    
    void icu4x_SentenceBreakIteratorUtf16_destroy_mv1(SentenceBreakIteratorUtf16* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t icu4x::SentenceBreakIteratorUtf16::next() {
  auto result = icu4x::capi::icu4x_SentenceBreakIteratorUtf16_next_mv1(this->AsFFI());
  return result;
}

inline const icu4x::capi::SentenceBreakIteratorUtf16* icu4x::SentenceBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const icu4x::capi::SentenceBreakIteratorUtf16*>(this);
}

inline icu4x::capi::SentenceBreakIteratorUtf16* icu4x::SentenceBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<icu4x::capi::SentenceBreakIteratorUtf16*>(this);
}

inline const icu4x::SentenceBreakIteratorUtf16* icu4x::SentenceBreakIteratorUtf16::FromFFI(const icu4x::capi::SentenceBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const icu4x::SentenceBreakIteratorUtf16*>(ptr);
}

inline icu4x::SentenceBreakIteratorUtf16* icu4x::SentenceBreakIteratorUtf16::FromFFI(icu4x::capi::SentenceBreakIteratorUtf16* ptr) {
  return reinterpret_cast<icu4x::SentenceBreakIteratorUtf16*>(ptr);
}

inline void icu4x::SentenceBreakIteratorUtf16::operator delete(void* ptr) {
  icu4x::capi::icu4x_SentenceBreakIteratorUtf16_destroy_mv1(reinterpret_cast<icu4x::capi::SentenceBreakIteratorUtf16*>(ptr));
}


#endif // icu4x_SentenceBreakIteratorUtf16_HPP
