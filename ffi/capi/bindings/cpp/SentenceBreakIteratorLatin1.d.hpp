#ifndef SentenceBreakIteratorLatin1_D_HPP
#define SentenceBreakIteratorLatin1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct SentenceBreakIteratorLatin1 SentenceBreakIteratorLatin1;
}

class SentenceBreakIteratorLatin1 {
public:

  inline int32_t next();

  inline const capi::SentenceBreakIteratorLatin1* AsFFI() const;
  inline capi::SentenceBreakIteratorLatin1* AsFFI();
  inline static const SentenceBreakIteratorLatin1* FromFFI(const capi::SentenceBreakIteratorLatin1* ptr);
  inline static SentenceBreakIteratorLatin1* FromFFI(capi::SentenceBreakIteratorLatin1* ptr);
  inline static void operator delete(void* ptr);
private:
  SentenceBreakIteratorLatin1() = delete;
  SentenceBreakIteratorLatin1(const SentenceBreakIteratorLatin1&) = delete;
  SentenceBreakIteratorLatin1(SentenceBreakIteratorLatin1&&) noexcept = delete;
  SentenceBreakIteratorLatin1 operator=(const SentenceBreakIteratorLatin1&) = delete;
  SentenceBreakIteratorLatin1 operator=(SentenceBreakIteratorLatin1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // SentenceBreakIteratorLatin1_D_HPP
