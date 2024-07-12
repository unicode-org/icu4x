#ifndef SentenceBreakIteratorUtf8_D_HPP
#define SentenceBreakIteratorUtf8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct SentenceBreakIteratorUtf8;
} // namespace capi
} // namespace

class SentenceBreakIteratorUtf8 {
public:

  inline int32_t next();

  inline const diplomat::capi::SentenceBreakIteratorUtf8* AsFFI() const;
  inline diplomat::capi::SentenceBreakIteratorUtf8* AsFFI();
  inline static const SentenceBreakIteratorUtf8* FromFFI(const diplomat::capi::SentenceBreakIteratorUtf8* ptr);
  inline static SentenceBreakIteratorUtf8* FromFFI(diplomat::capi::SentenceBreakIteratorUtf8* ptr);
  inline static void operator delete(void* ptr);
private:
  SentenceBreakIteratorUtf8() = delete;
  SentenceBreakIteratorUtf8(const SentenceBreakIteratorUtf8&) = delete;
  SentenceBreakIteratorUtf8(SentenceBreakIteratorUtf8&&) noexcept = delete;
  SentenceBreakIteratorUtf8 operator=(const SentenceBreakIteratorUtf8&) = delete;
  SentenceBreakIteratorUtf8 operator=(SentenceBreakIteratorUtf8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // SentenceBreakIteratorUtf8_D_HPP
