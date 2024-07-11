#ifndef SentenceBreakIteratorUtf16_HPP
#define SentenceBreakIteratorUtf16_HPP

#include "SentenceBreakIteratorUtf16.d.hpp"

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
    
    int32_t ICU4XSentenceBreakIteratorUtf16_next(diplomat::capi::SentenceBreakIteratorUtf16* self);
    
    
    void ICU4XSentenceBreakIteratorUtf16_destroy(SentenceBreakIteratorUtf16* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t SentenceBreakIteratorUtf16::next() {
  auto result = diplomat::capi::ICU4XSentenceBreakIteratorUtf16_next(this->AsFFI());
  return result;
}

inline const diplomat::capi::SentenceBreakIteratorUtf16* SentenceBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::SentenceBreakIteratorUtf16*>(this);
}

inline diplomat::capi::SentenceBreakIteratorUtf16* SentenceBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<diplomat::capi::SentenceBreakIteratorUtf16*>(this);
}

inline const SentenceBreakIteratorUtf16* SentenceBreakIteratorUtf16::FromFFI(const diplomat::capi::SentenceBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const SentenceBreakIteratorUtf16*>(ptr);
}

inline SentenceBreakIteratorUtf16* SentenceBreakIteratorUtf16::FromFFI(diplomat::capi::SentenceBreakIteratorUtf16* ptr) {
  return reinterpret_cast<SentenceBreakIteratorUtf16*>(ptr);
}

inline void SentenceBreakIteratorUtf16::operator delete(void* ptr) {
  diplomat::capi::ICU4XSentenceBreakIteratorUtf16_destroy(reinterpret_cast<diplomat::capi::SentenceBreakIteratorUtf16*>(ptr));
}


#endif // SentenceBreakIteratorUtf16_HPP
