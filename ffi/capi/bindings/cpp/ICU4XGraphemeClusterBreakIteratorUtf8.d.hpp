#ifndef ICU4XGraphemeClusterBreakIteratorUtf8_D_HPP
#define ICU4XGraphemeClusterBreakIteratorUtf8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct ICU4XGraphemeClusterBreakIteratorUtf8 ICU4XGraphemeClusterBreakIteratorUtf8;
}

class ICU4XGraphemeClusterBreakIteratorUtf8 {
public:

  inline int32_t next();

  inline const capi::ICU4XGraphemeClusterBreakIteratorUtf8* AsFFI() const;
  inline capi::ICU4XGraphemeClusterBreakIteratorUtf8* AsFFI();
  inline static const ICU4XGraphemeClusterBreakIteratorUtf8* FromFFI(const capi::ICU4XGraphemeClusterBreakIteratorUtf8* ptr);
  inline static ICU4XGraphemeClusterBreakIteratorUtf8* FromFFI(capi::ICU4XGraphemeClusterBreakIteratorUtf8* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XGraphemeClusterBreakIteratorUtf8() = delete;
  ICU4XGraphemeClusterBreakIteratorUtf8(const ICU4XGraphemeClusterBreakIteratorUtf8&) = delete;
  ICU4XGraphemeClusterBreakIteratorUtf8(ICU4XGraphemeClusterBreakIteratorUtf8&&) noexcept = delete;
  ICU4XGraphemeClusterBreakIteratorUtf8 operator=(const ICU4XGraphemeClusterBreakIteratorUtf8&) = delete;
  ICU4XGraphemeClusterBreakIteratorUtf8 operator=(ICU4XGraphemeClusterBreakIteratorUtf8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XGraphemeClusterBreakIteratorUtf8_D_HPP
