#ifndef ICU4XSentenceBreakIteratorLatin1_D_HPP
#define ICU4XSentenceBreakIteratorLatin1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XSentenceBreakIteratorLatin1.d.h"


class ICU4XSentenceBreakIteratorLatin1 {
public:

  inline int32_t next();

  inline const capi::ICU4XSentenceBreakIteratorLatin1* AsFFI() const;
  inline capi::ICU4XSentenceBreakIteratorLatin1* AsFFI();
  inline static const ICU4XSentenceBreakIteratorLatin1* FromFFI(const capi::ICU4XSentenceBreakIteratorLatin1* ptr);
  inline static ICU4XSentenceBreakIteratorLatin1* FromFFI(capi::ICU4XSentenceBreakIteratorLatin1* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XSentenceBreakIteratorLatin1() = delete;
  ICU4XSentenceBreakIteratorLatin1(const ICU4XSentenceBreakIteratorLatin1&) = delete;
  ICU4XSentenceBreakIteratorLatin1(ICU4XSentenceBreakIteratorLatin1&&) noexcept = delete;
  ICU4XSentenceBreakIteratorLatin1 operator=(const ICU4XSentenceBreakIteratorLatin1&) = delete;
  ICU4XSentenceBreakIteratorLatin1 operator=(ICU4XSentenceBreakIteratorLatin1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XSentenceBreakIteratorLatin1_D_HPP
