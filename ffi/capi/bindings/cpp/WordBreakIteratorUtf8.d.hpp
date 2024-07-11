#ifndef WordBreakIteratorUtf8_D_HPP
#define WordBreakIteratorUtf8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "SegmenterWordType.d.hpp"

class SegmenterWordType;


namespace capi {
    typedef struct WordBreakIteratorUtf8 WordBreakIteratorUtf8;
}

class WordBreakIteratorUtf8 {
public:

  inline int32_t next();

  inline SegmenterWordType word_type() const;

  inline bool is_word_like() const;

  inline const capi::WordBreakIteratorUtf8* AsFFI() const;
  inline capi::WordBreakIteratorUtf8* AsFFI();
  inline static const WordBreakIteratorUtf8* FromFFI(const capi::WordBreakIteratorUtf8* ptr);
  inline static WordBreakIteratorUtf8* FromFFI(capi::WordBreakIteratorUtf8* ptr);
  inline static void operator delete(void* ptr);
private:
  WordBreakIteratorUtf8() = delete;
  WordBreakIteratorUtf8(const WordBreakIteratorUtf8&) = delete;
  WordBreakIteratorUtf8(WordBreakIteratorUtf8&&) noexcept = delete;
  WordBreakIteratorUtf8 operator=(const WordBreakIteratorUtf8&) = delete;
  WordBreakIteratorUtf8 operator=(WordBreakIteratorUtf8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // WordBreakIteratorUtf8_D_HPP
