#ifndef ICU4XGraphemeClusterBreakIteratorUtf16_D_HPP
#define ICU4XGraphemeClusterBreakIteratorUtf16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XGraphemeClusterBreakIteratorUtf16.d.h"


class ICU4XGraphemeClusterBreakIteratorUtf16 {
public:

  inline int32_t next();

  inline const capi::ICU4XGraphemeClusterBreakIteratorUtf16* AsFFI() const;
  inline capi::ICU4XGraphemeClusterBreakIteratorUtf16* AsFFI();
  inline static const ICU4XGraphemeClusterBreakIteratorUtf16* FromFFI(const capi::ICU4XGraphemeClusterBreakIteratorUtf16* ptr);
  inline static ICU4XGraphemeClusterBreakIteratorUtf16* FromFFI(capi::ICU4XGraphemeClusterBreakIteratorUtf16* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XGraphemeClusterBreakIteratorUtf16() = delete;
  ICU4XGraphemeClusterBreakIteratorUtf16(const ICU4XGraphemeClusterBreakIteratorUtf16&) = delete;
  ICU4XGraphemeClusterBreakIteratorUtf16(ICU4XGraphemeClusterBreakIteratorUtf16&&) noexcept = delete;
  ICU4XGraphemeClusterBreakIteratorUtf16 operator=(const ICU4XGraphemeClusterBreakIteratorUtf16&) = delete;
  ICU4XGraphemeClusterBreakIteratorUtf16 operator=(ICU4XGraphemeClusterBreakIteratorUtf16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XGraphemeClusterBreakIteratorUtf16_D_HPP
