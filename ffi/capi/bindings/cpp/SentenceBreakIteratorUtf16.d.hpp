#ifndef SentenceBreakIteratorUtf16_D_HPP
#define SentenceBreakIteratorUtf16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct SentenceBreakIteratorUtf16;
} // namespace capi
} // namespace

class SentenceBreakIteratorUtf16 {
public:

  inline int32_t next();

  inline const diplomat::capi::SentenceBreakIteratorUtf16* AsFFI() const;
  inline diplomat::capi::SentenceBreakIteratorUtf16* AsFFI();
  inline static const SentenceBreakIteratorUtf16* FromFFI(const diplomat::capi::SentenceBreakIteratorUtf16* ptr);
  inline static SentenceBreakIteratorUtf16* FromFFI(diplomat::capi::SentenceBreakIteratorUtf16* ptr);
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
