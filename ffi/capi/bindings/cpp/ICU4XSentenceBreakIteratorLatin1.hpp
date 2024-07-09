#ifndef ICU4XSentenceBreakIteratorLatin1_HPP
#define ICU4XSentenceBreakIteratorLatin1_HPP

#include "ICU4XSentenceBreakIteratorLatin1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    int32_t ICU4XSentenceBreakIteratorLatin1_next(ICU4XSentenceBreakIteratorLatin1* self);
    
    
    void ICU4XSentenceBreakIteratorLatin1_destroy(ICU4XSentenceBreakIteratorLatin1* self);
    
    } // extern "C"
}

inline int32_t ICU4XSentenceBreakIteratorLatin1::next() {
  auto result = capi::ICU4XSentenceBreakIteratorLatin1_next(this->AsFFI());
  return result;
}

inline const capi::ICU4XSentenceBreakIteratorLatin1* ICU4XSentenceBreakIteratorLatin1::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XSentenceBreakIteratorLatin1*>(this);
}

inline capi::ICU4XSentenceBreakIteratorLatin1* ICU4XSentenceBreakIteratorLatin1::AsFFI() {
  return reinterpret_cast<capi::ICU4XSentenceBreakIteratorLatin1*>(this);
}

inline const ICU4XSentenceBreakIteratorLatin1* ICU4XSentenceBreakIteratorLatin1::FromFFI(const capi::ICU4XSentenceBreakIteratorLatin1* ptr) {
  return reinterpret_cast<const ICU4XSentenceBreakIteratorLatin1*>(ptr);
}

inline ICU4XSentenceBreakIteratorLatin1* ICU4XSentenceBreakIteratorLatin1::FromFFI(capi::ICU4XSentenceBreakIteratorLatin1* ptr) {
  return reinterpret_cast<ICU4XSentenceBreakIteratorLatin1*>(ptr);
}

inline void ICU4XSentenceBreakIteratorLatin1::operator delete(void* ptr) {
  capi::ICU4XSentenceBreakIteratorLatin1_destroy(reinterpret_cast<capi::ICU4XSentenceBreakIteratorLatin1*>(ptr));
}


#endif // ICU4XSentenceBreakIteratorLatin1_HPP
