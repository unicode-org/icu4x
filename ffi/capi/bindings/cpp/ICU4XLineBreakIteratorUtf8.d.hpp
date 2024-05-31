#ifndef ICU4XLineBreakIteratorUtf8_D_HPP
#define ICU4XLineBreakIteratorUtf8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLineBreakIteratorUtf8.d.h"


class ICU4XLineBreakIteratorUtf8 {
public:

  inline int32_t next();

  inline const capi::ICU4XLineBreakIteratorUtf8* AsFFI() const;
  inline capi::ICU4XLineBreakIteratorUtf8* AsFFI();
  inline static const ICU4XLineBreakIteratorUtf8* FromFFI(const capi::ICU4XLineBreakIteratorUtf8* ptr);
  inline static ICU4XLineBreakIteratorUtf8* FromFFI(capi::ICU4XLineBreakIteratorUtf8* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLineBreakIteratorUtf8() = delete;
  ICU4XLineBreakIteratorUtf8(const ICU4XLineBreakIteratorUtf8&) = delete;
  ICU4XLineBreakIteratorUtf8(ICU4XLineBreakIteratorUtf8&&) noexcept = delete;
  ICU4XLineBreakIteratorUtf8 operator=(const ICU4XLineBreakIteratorUtf8&) = delete;
  ICU4XLineBreakIteratorUtf8 operator=(ICU4XLineBreakIteratorUtf8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLineBreakIteratorUtf8_D_HPP
