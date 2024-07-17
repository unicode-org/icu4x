#ifndef SentenceBreakIteratorUtf8_HPP
#define SentenceBreakIteratorUtf8_HPP

#include "SentenceBreakIteratorUtf8.d.hpp"

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
    
    int32_t icu4x_SentenceBreakIteratorUtf8_next_mv1(diplomat::capi::SentenceBreakIteratorUtf8* self);
    
    
    void icu4x_SentenceBreakIteratorUtf8_destroy_mv1(SentenceBreakIteratorUtf8* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t SentenceBreakIteratorUtf8::next() {
  auto result = diplomat::capi::icu4x_SentenceBreakIteratorUtf8_next_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::SentenceBreakIteratorUtf8* SentenceBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::SentenceBreakIteratorUtf8*>(this);
}

inline diplomat::capi::SentenceBreakIteratorUtf8* SentenceBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<diplomat::capi::SentenceBreakIteratorUtf8*>(this);
}

inline const SentenceBreakIteratorUtf8* SentenceBreakIteratorUtf8::FromFFI(const diplomat::capi::SentenceBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const SentenceBreakIteratorUtf8*>(ptr);
}

inline SentenceBreakIteratorUtf8* SentenceBreakIteratorUtf8::FromFFI(diplomat::capi::SentenceBreakIteratorUtf8* ptr) {
  return reinterpret_cast<SentenceBreakIteratorUtf8*>(ptr);
}

inline void SentenceBreakIteratorUtf8::operator delete(void* ptr) {
  diplomat::capi::icu4x_SentenceBreakIteratorUtf8_destroy_mv1(reinterpret_cast<diplomat::capi::SentenceBreakIteratorUtf8*>(ptr));
}


#endif // SentenceBreakIteratorUtf8_HPP
