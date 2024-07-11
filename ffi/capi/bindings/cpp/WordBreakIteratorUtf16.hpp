#ifndef WordBreakIteratorUtf16_HPP
#define WordBreakIteratorUtf16_HPP

#include "WordBreakIteratorUtf16.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "SegmenterWordType.hpp"


namespace capi {
    extern "C" {
    
    int32_t ICU4XWordBreakIteratorUtf16_next(WordBreakIteratorUtf16* self);
    
    SegmenterWordType ICU4XWordBreakIteratorUtf16_word_type(const WordBreakIteratorUtf16* self);
    
    bool ICU4XWordBreakIteratorUtf16_is_word_like(const WordBreakIteratorUtf16* self);
    
    
    void ICU4XWordBreakIteratorUtf16_destroy(WordBreakIteratorUtf16* self);
    
    } // extern "C"
}

inline int32_t WordBreakIteratorUtf16::next() {
  auto result = capi::ICU4XWordBreakIteratorUtf16_next(this->AsFFI());
  return result;
}

inline SegmenterWordType WordBreakIteratorUtf16::word_type() const {
  auto result = capi::ICU4XWordBreakIteratorUtf16_word_type(this->AsFFI());
  return SegmenterWordType::FromFFI(result);
}

inline bool WordBreakIteratorUtf16::is_word_like() const {
  auto result = capi::ICU4XWordBreakIteratorUtf16_is_word_like(this->AsFFI());
  return result;
}

inline const capi::WordBreakIteratorUtf16* WordBreakIteratorUtf16::AsFFI() const {
  return reinterpret_cast<const capi::WordBreakIteratorUtf16*>(this);
}

inline capi::WordBreakIteratorUtf16* WordBreakIteratorUtf16::AsFFI() {
  return reinterpret_cast<capi::WordBreakIteratorUtf16*>(this);
}

inline const WordBreakIteratorUtf16* WordBreakIteratorUtf16::FromFFI(const capi::WordBreakIteratorUtf16* ptr) {
  return reinterpret_cast<const WordBreakIteratorUtf16*>(ptr);
}

inline WordBreakIteratorUtf16* WordBreakIteratorUtf16::FromFFI(capi::WordBreakIteratorUtf16* ptr) {
  return reinterpret_cast<WordBreakIteratorUtf16*>(ptr);
}

inline void WordBreakIteratorUtf16::operator delete(void* ptr) {
  capi::ICU4XWordBreakIteratorUtf16_destroy(reinterpret_cast<capi::WordBreakIteratorUtf16*>(ptr));
}


#endif // WordBreakIteratorUtf16_HPP
