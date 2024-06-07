#ifndef ICU4XLineBreakIteratorUtf16_D_HPP
#define ICU4XLineBreakIteratorUtf16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLineBreakIteratorUtf16.d.h"


class ICU4XLineBreakIteratorUtf16 {
public:

  inline int32_t next();

  inline const capi::ICU4XLineBreakIteratorUtf16* AsFFI() const;
  inline capi::ICU4XLineBreakIteratorUtf16* AsFFI();
  inline static const ICU4XLineBreakIteratorUtf16* FromFFI(const capi::ICU4XLineBreakIteratorUtf16* ptr);
  inline static ICU4XLineBreakIteratorUtf16* FromFFI(capi::ICU4XLineBreakIteratorUtf16* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLineBreakIteratorUtf16() = delete;
  ICU4XLineBreakIteratorUtf16(const ICU4XLineBreakIteratorUtf16&) = delete;
  ICU4XLineBreakIteratorUtf16(ICU4XLineBreakIteratorUtf16&&) noexcept = delete;
  ICU4XLineBreakIteratorUtf16 operator=(const ICU4XLineBreakIteratorUtf16&) = delete;
  ICU4XLineBreakIteratorUtf16 operator=(ICU4XLineBreakIteratorUtf16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLineBreakIteratorUtf16_D_HPP
