#ifndef ICU4XGraphemeClusterBreakIteratorLatin1_D_HPP
#define ICU4XGraphemeClusterBreakIteratorLatin1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct ICU4XGraphemeClusterBreakIteratorLatin1 ICU4XGraphemeClusterBreakIteratorLatin1;
}

class ICU4XGraphemeClusterBreakIteratorLatin1 {
public:

  inline int32_t next();

  inline const capi::ICU4XGraphemeClusterBreakIteratorLatin1* AsFFI() const;
  inline capi::ICU4XGraphemeClusterBreakIteratorLatin1* AsFFI();
  inline static const ICU4XGraphemeClusterBreakIteratorLatin1* FromFFI(const capi::ICU4XGraphemeClusterBreakIteratorLatin1* ptr);
  inline static ICU4XGraphemeClusterBreakIteratorLatin1* FromFFI(capi::ICU4XGraphemeClusterBreakIteratorLatin1* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XGraphemeClusterBreakIteratorLatin1() = delete;
  ICU4XGraphemeClusterBreakIteratorLatin1(const ICU4XGraphemeClusterBreakIteratorLatin1&) = delete;
  ICU4XGraphemeClusterBreakIteratorLatin1(ICU4XGraphemeClusterBreakIteratorLatin1&&) noexcept = delete;
  ICU4XGraphemeClusterBreakIteratorLatin1 operator=(const ICU4XGraphemeClusterBreakIteratorLatin1&) = delete;
  ICU4XGraphemeClusterBreakIteratorLatin1 operator=(ICU4XGraphemeClusterBreakIteratorLatin1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XGraphemeClusterBreakIteratorLatin1_D_HPP
