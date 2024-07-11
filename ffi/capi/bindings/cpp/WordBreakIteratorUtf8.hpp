#ifndef WordBreakIteratorUtf8_HPP
#define WordBreakIteratorUtf8_HPP

#include "WordBreakIteratorUtf8.d.hpp"

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
    
    int32_t ICU4XWordBreakIteratorUtf8_next(WordBreakIteratorUtf8* self);
    
    SegmenterWordType ICU4XWordBreakIteratorUtf8_word_type(const WordBreakIteratorUtf8* self);
    
    bool ICU4XWordBreakIteratorUtf8_is_word_like(const WordBreakIteratorUtf8* self);
    
    
    void ICU4XWordBreakIteratorUtf8_destroy(WordBreakIteratorUtf8* self);
    
    } // extern "C"
}

inline int32_t WordBreakIteratorUtf8::next() {
  auto result = capi::ICU4XWordBreakIteratorUtf8_next(this->AsFFI());
  return result;
}

inline SegmenterWordType WordBreakIteratorUtf8::word_type() const {
  auto result = capi::ICU4XWordBreakIteratorUtf8_word_type(this->AsFFI());
  return SegmenterWordType::FromFFI(result);
}

inline bool WordBreakIteratorUtf8::is_word_like() const {
  auto result = capi::ICU4XWordBreakIteratorUtf8_is_word_like(this->AsFFI());
  return result;
}

inline const capi::WordBreakIteratorUtf8* WordBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const capi::WordBreakIteratorUtf8*>(this);
}

inline capi::WordBreakIteratorUtf8* WordBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<capi::WordBreakIteratorUtf8*>(this);
}

inline const WordBreakIteratorUtf8* WordBreakIteratorUtf8::FromFFI(const capi::WordBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const WordBreakIteratorUtf8*>(ptr);
}

inline WordBreakIteratorUtf8* WordBreakIteratorUtf8::FromFFI(capi::WordBreakIteratorUtf8* ptr) {
  return reinterpret_cast<WordBreakIteratorUtf8*>(ptr);
}

inline void WordBreakIteratorUtf8::operator delete(void* ptr) {
  capi::ICU4XWordBreakIteratorUtf8_destroy(reinterpret_cast<capi::WordBreakIteratorUtf8*>(ptr));
}


#endif // WordBreakIteratorUtf8_HPP
