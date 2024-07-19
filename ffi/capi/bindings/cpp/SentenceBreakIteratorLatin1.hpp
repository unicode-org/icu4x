#ifndef SentenceBreakIteratorLatin1_HPP
#define SentenceBreakIteratorLatin1_HPP

#include "SentenceBreakIteratorLatin1.d.hpp"

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
    
    int32_t icu4x_SentenceBreakIteratorLatin1_next_mv1(diplomat::capi::SentenceBreakIteratorLatin1* self);
    
    
    void icu4x_SentenceBreakIteratorLatin1_destroy_mv1(SentenceBreakIteratorLatin1* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t SentenceBreakIteratorLatin1::next() {
  auto result = diplomat::capi::icu4x_SentenceBreakIteratorLatin1_next_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::SentenceBreakIteratorLatin1* SentenceBreakIteratorLatin1::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::SentenceBreakIteratorLatin1*>(this);
}

inline diplomat::capi::SentenceBreakIteratorLatin1* SentenceBreakIteratorLatin1::AsFFI() {
  return reinterpret_cast<diplomat::capi::SentenceBreakIteratorLatin1*>(this);
}

inline const SentenceBreakIteratorLatin1* SentenceBreakIteratorLatin1::FromFFI(const diplomat::capi::SentenceBreakIteratorLatin1* ptr) {
  return reinterpret_cast<const SentenceBreakIteratorLatin1*>(ptr);
}

inline SentenceBreakIteratorLatin1* SentenceBreakIteratorLatin1::FromFFI(diplomat::capi::SentenceBreakIteratorLatin1* ptr) {
  return reinterpret_cast<SentenceBreakIteratorLatin1*>(ptr);
}

inline void SentenceBreakIteratorLatin1::operator delete(void* ptr) {
  diplomat::capi::icu4x_SentenceBreakIteratorLatin1_destroy_mv1(reinterpret_cast<diplomat::capi::SentenceBreakIteratorLatin1*>(ptr));
}


#endif // SentenceBreakIteratorLatin1_HPP
