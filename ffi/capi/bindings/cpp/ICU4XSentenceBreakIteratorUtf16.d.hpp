#ifndef ICU4XSentenceBreakIteratorUtf16_D_HPP
#define ICU4XSentenceBreakIteratorUtf16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct ICU4XSentenceBreakIteratorUtf16 ICU4XSentenceBreakIteratorUtf16;
}

class ICU4XSentenceBreakIteratorUtf16 {
public:

  inline int32_t next();

  inline const capi::ICU4XSentenceBreakIteratorUtf16* AsFFI() const;
  inline capi::ICU4XSentenceBreakIteratorUtf16* AsFFI();
  inline static const ICU4XSentenceBreakIteratorUtf16* FromFFI(const capi::ICU4XSentenceBreakIteratorUtf16* ptr);
  inline static ICU4XSentenceBreakIteratorUtf16* FromFFI(capi::ICU4XSentenceBreakIteratorUtf16* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XSentenceBreakIteratorUtf16() = delete;
  ICU4XSentenceBreakIteratorUtf16(const ICU4XSentenceBreakIteratorUtf16&) = delete;
  ICU4XSentenceBreakIteratorUtf16(ICU4XSentenceBreakIteratorUtf16&&) noexcept = delete;
  ICU4XSentenceBreakIteratorUtf16 operator=(const ICU4XSentenceBreakIteratorUtf16&) = delete;
  ICU4XSentenceBreakIteratorUtf16 operator=(ICU4XSentenceBreakIteratorUtf16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XSentenceBreakIteratorUtf16_D_HPP
