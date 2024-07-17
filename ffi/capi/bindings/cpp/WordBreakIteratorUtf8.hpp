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


namespace diplomat {
namespace capi {
    extern "C" {
    
    int32_t icu4x_WordBreakIteratorUtf8_next_mv1(diplomat::capi::WordBreakIteratorUtf8* self);
    
    diplomat::capi::SegmenterWordType icu4x_WordBreakIteratorUtf8_word_type_mv1(const diplomat::capi::WordBreakIteratorUtf8* self);
    
    bool icu4x_WordBreakIteratorUtf8_is_word_like_mv1(const diplomat::capi::WordBreakIteratorUtf8* self);
    
    
    void icu4x_WordBreakIteratorUtf8_destroy_mv1(WordBreakIteratorUtf8* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t WordBreakIteratorUtf8::next() {
  auto result = diplomat::capi::icu4x_WordBreakIteratorUtf8_next_mv1(this->AsFFI());
  return result;
}

inline SegmenterWordType WordBreakIteratorUtf8::word_type() const {
  auto result = diplomat::capi::icu4x_WordBreakIteratorUtf8_word_type_mv1(this->AsFFI());
  return SegmenterWordType::FromFFI(result);
}

inline bool WordBreakIteratorUtf8::is_word_like() const {
  auto result = diplomat::capi::icu4x_WordBreakIteratorUtf8_is_word_like_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::WordBreakIteratorUtf8* WordBreakIteratorUtf8::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::WordBreakIteratorUtf8*>(this);
}

inline diplomat::capi::WordBreakIteratorUtf8* WordBreakIteratorUtf8::AsFFI() {
  return reinterpret_cast<diplomat::capi::WordBreakIteratorUtf8*>(this);
}

inline const WordBreakIteratorUtf8* WordBreakIteratorUtf8::FromFFI(const diplomat::capi::WordBreakIteratorUtf8* ptr) {
  return reinterpret_cast<const WordBreakIteratorUtf8*>(ptr);
}

inline WordBreakIteratorUtf8* WordBreakIteratorUtf8::FromFFI(diplomat::capi::WordBreakIteratorUtf8* ptr) {
  return reinterpret_cast<WordBreakIteratorUtf8*>(ptr);
}

inline void WordBreakIteratorUtf8::operator delete(void* ptr) {
  diplomat::capi::icu4x_WordBreakIteratorUtf8_destroy_mv1(reinterpret_cast<diplomat::capi::WordBreakIteratorUtf8*>(ptr));
}


#endif // WordBreakIteratorUtf8_HPP
