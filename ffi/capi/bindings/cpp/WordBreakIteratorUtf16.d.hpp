#ifndef WordBreakIteratorUtf16_D_HPP
#define WordBreakIteratorUtf16_D_HPP

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
    typedef struct WordBreakIteratorUtf16 WordBreakIteratorUtf16;
}

class WordBreakIteratorUtf16 {
public:

  inline int32_t next();

  inline SegmenterWordType word_type() const;

  inline bool is_word_like() const;

  inline const capi::WordBreakIteratorUtf16* AsFFI() const;
  inline capi::WordBreakIteratorUtf16* AsFFI();
  inline static const WordBreakIteratorUtf16* FromFFI(const capi::WordBreakIteratorUtf16* ptr);
  inline static WordBreakIteratorUtf16* FromFFI(capi::WordBreakIteratorUtf16* ptr);
  inline static void operator delete(void* ptr);
private:
  WordBreakIteratorUtf16() = delete;
  WordBreakIteratorUtf16(const WordBreakIteratorUtf16&) = delete;
  WordBreakIteratorUtf16(WordBreakIteratorUtf16&&) noexcept = delete;
  WordBreakIteratorUtf16 operator=(const WordBreakIteratorUtf16&) = delete;
  WordBreakIteratorUtf16 operator=(WordBreakIteratorUtf16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // WordBreakIteratorUtf16_D_HPP
