#ifndef WordBreakIteratorLatin1_D_HPP
#define WordBreakIteratorLatin1_D_HPP

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
    typedef struct WordBreakIteratorLatin1 WordBreakIteratorLatin1;
}

class WordBreakIteratorLatin1 {
public:

  inline int32_t next();

  inline SegmenterWordType word_type() const;

  inline bool is_word_like() const;

  inline const capi::WordBreakIteratorLatin1* AsFFI() const;
  inline capi::WordBreakIteratorLatin1* AsFFI();
  inline static const WordBreakIteratorLatin1* FromFFI(const capi::WordBreakIteratorLatin1* ptr);
  inline static WordBreakIteratorLatin1* FromFFI(capi::WordBreakIteratorLatin1* ptr);
  inline static void operator delete(void* ptr);
private:
  WordBreakIteratorLatin1() = delete;
  WordBreakIteratorLatin1(const WordBreakIteratorLatin1&) = delete;
  WordBreakIteratorLatin1(WordBreakIteratorLatin1&&) noexcept = delete;
  WordBreakIteratorLatin1 operator=(const WordBreakIteratorLatin1&) = delete;
  WordBreakIteratorLatin1 operator=(WordBreakIteratorLatin1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // WordBreakIteratorLatin1_D_HPP
