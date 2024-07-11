#ifndef SentenceBreakIteratorUtf16_D_HPP
#define SentenceBreakIteratorUtf16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct SentenceBreakIteratorUtf16 SentenceBreakIteratorUtf16;
}

class SentenceBreakIteratorUtf16 {
public:

  inline int32_t next();

  inline const capi::SentenceBreakIteratorUtf16* AsFFI() const;
  inline capi::SentenceBreakIteratorUtf16* AsFFI();
  inline static const SentenceBreakIteratorUtf16* FromFFI(const capi::SentenceBreakIteratorUtf16* ptr);
  inline static SentenceBreakIteratorUtf16* FromFFI(capi::SentenceBreakIteratorUtf16* ptr);
  inline static void operator delete(void* ptr);
private:
  SentenceBreakIteratorUtf16() = delete;
  SentenceBreakIteratorUtf16(const SentenceBreakIteratorUtf16&) = delete;
  SentenceBreakIteratorUtf16(SentenceBreakIteratorUtf16&&) noexcept = delete;
  SentenceBreakIteratorUtf16 operator=(const SentenceBreakIteratorUtf16&) = delete;
  SentenceBreakIteratorUtf16 operator=(SentenceBreakIteratorUtf16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // SentenceBreakIteratorUtf16_D_HPP
