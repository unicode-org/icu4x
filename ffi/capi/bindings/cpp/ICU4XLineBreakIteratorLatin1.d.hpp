#ifndef ICU4XLineBreakIteratorLatin1_D_HPP
#define ICU4XLineBreakIteratorLatin1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct ICU4XLineBreakIteratorLatin1 ICU4XLineBreakIteratorLatin1;
}

class ICU4XLineBreakIteratorLatin1 {
public:

  inline int32_t next();

  inline const capi::ICU4XLineBreakIteratorLatin1* AsFFI() const;
  inline capi::ICU4XLineBreakIteratorLatin1* AsFFI();
  inline static const ICU4XLineBreakIteratorLatin1* FromFFI(const capi::ICU4XLineBreakIteratorLatin1* ptr);
  inline static ICU4XLineBreakIteratorLatin1* FromFFI(capi::ICU4XLineBreakIteratorLatin1* ptr);
  inline static void operator delete(void* ptr);
private:
  ICU4XLineBreakIteratorLatin1() = delete;
  ICU4XLineBreakIteratorLatin1(const ICU4XLineBreakIteratorLatin1&) = delete;
  ICU4XLineBreakIteratorLatin1(ICU4XLineBreakIteratorLatin1&&) noexcept = delete;
  ICU4XLineBreakIteratorLatin1 operator=(const ICU4XLineBreakIteratorLatin1&) = delete;
  ICU4XLineBreakIteratorLatin1 operator=(ICU4XLineBreakIteratorLatin1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // ICU4XLineBreakIteratorLatin1_D_HPP
