#ifndef WordBreakIteratorLatin1_HPP
#define WordBreakIteratorLatin1_HPP

#include "WordBreakIteratorLatin1.d.hpp"

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
    
    int32_t icu4x_WordBreakIteratorLatin1_next_mv1(diplomat::capi::WordBreakIteratorLatin1* self);
    
    diplomat::capi::SegmenterWordType icu4x_WordBreakIteratorLatin1_word_type_mv1(const diplomat::capi::WordBreakIteratorLatin1* self);
    
    bool icu4x_WordBreakIteratorLatin1_is_word_like_mv1(const diplomat::capi::WordBreakIteratorLatin1* self);
    
    
    void icu4x_WordBreakIteratorLatin1_destroy_mv1(WordBreakIteratorLatin1* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline int32_t WordBreakIteratorLatin1::next() {
  auto result = diplomat::capi::icu4x_WordBreakIteratorLatin1_next_mv1(this->AsFFI());
  return result;
}

inline SegmenterWordType WordBreakIteratorLatin1::word_type() const {
  auto result = diplomat::capi::icu4x_WordBreakIteratorLatin1_word_type_mv1(this->AsFFI());
  return SegmenterWordType::FromFFI(result);
}

inline bool WordBreakIteratorLatin1::is_word_like() const {
  auto result = diplomat::capi::icu4x_WordBreakIteratorLatin1_is_word_like_mv1(this->AsFFI());
  return result;
}

inline const diplomat::capi::WordBreakIteratorLatin1* WordBreakIteratorLatin1::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::WordBreakIteratorLatin1*>(this);
}

inline diplomat::capi::WordBreakIteratorLatin1* WordBreakIteratorLatin1::AsFFI() {
  return reinterpret_cast<diplomat::capi::WordBreakIteratorLatin1*>(this);
}

inline const WordBreakIteratorLatin1* WordBreakIteratorLatin1::FromFFI(const diplomat::capi::WordBreakIteratorLatin1* ptr) {
  return reinterpret_cast<const WordBreakIteratorLatin1*>(ptr);
}

inline WordBreakIteratorLatin1* WordBreakIteratorLatin1::FromFFI(diplomat::capi::WordBreakIteratorLatin1* ptr) {
  return reinterpret_cast<WordBreakIteratorLatin1*>(ptr);
}

inline void WordBreakIteratorLatin1::operator delete(void* ptr) {
  diplomat::capi::icu4x_WordBreakIteratorLatin1_destroy_mv1(reinterpret_cast<diplomat::capi::WordBreakIteratorLatin1*>(ptr));
}


#endif // WordBreakIteratorLatin1_HPP
