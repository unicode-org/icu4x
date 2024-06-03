#ifndef ICU4XSentenceBreakIteratorUtf8_D_HPP
#define ICU4XSentenceBreakIteratorUtf8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XSentenceBreakIteratorUtf8.d.h"


class ICU4XSentenceBreakIteratorUtf8 {
public:

  inline int32_t next();

  inline const capi::ICU4XSentenceBreakIteratorUtf8* AsFFI() const;
  inline capi::ICU4XSentenceBreakIteratorUtf8* AsFFI();
  inline static const ICU4XSentenceBreakIteratorUtf8* FromFFI(const capi::ICU4XSentenceBreakIteratorUtf8* ptr);
  inline static ICU4XSentenceBreakIteratorUtf8* FromFFI(capi::ICU4XSentenceBreakIteratorUtf8* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XSentenceBreakIteratorUtf8() = delete;
  ICU4XSentenceBreakIteratorUtf8(const ICU4XSentenceBreakIteratorUtf8&) = delete;
  ICU4XSentenceBreakIteratorUtf8(ICU4XSentenceBreakIteratorUtf8&&) noexcept = delete;
  ICU4XSentenceBreakIteratorUtf8 operator=(const ICU4XSentenceBreakIteratorUtf8&) = delete;
  ICU4XSentenceBreakIteratorUtf8 operator=(ICU4XSentenceBreakIteratorUtf8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XSentenceBreakIteratorUtf8_D_HPP
