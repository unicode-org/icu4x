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


namespace capi {
    extern "C" {
    
    int32_t ICU4XSentenceBreakIteratorUtf8_next(SentenceBreakIteratorUtf8* self);
    
    
    void ICU4XSentenceBreakIteratorUtf8_destroy(SentenceBreakIteratorUtf8* self);
    
    } // extern "C"
}

inline int32_t SentenceBreakIteratorUtf8::next() {
  auto result = capi::ICU4XSentenceBreakIteratorUtf8_next(this->AsFFI());
  return result;
}

inline const capi::SentenceBreakIteratorUtf8* SentenceBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const capi::SentenceBreakIteratorUtf8*>(this);
}

inline capi::SentenceBreakIteratorUtf8* SentenceBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<capi::SentenceBreakIteratorUtf8*>(this);
}

inline const SentenceBreakIteratorUtf8* SentenceBreakIteratorUtf8::FromFFI(const capi::SentenceBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const SentenceBreakIteratorUtf8*>(ptr);
}

inline SentenceBreakIteratorUtf8* SentenceBreakIteratorUtf8::FromFFI(capi::SentenceBreakIteratorUtf8* ptr) {
  return reinterpret_cast<SentenceBreakIteratorUtf8*>(ptr);
}

inline void SentenceBreakIteratorUtf8::operator delete(void* ptr) {
  capi::ICU4XSentenceBreakIteratorUtf8_destroy(reinterpret_cast<capi::SentenceBreakIteratorUtf8*>(ptr));
}


#endif // SentenceBreakIteratorUtf8_HPP
