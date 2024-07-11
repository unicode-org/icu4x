#ifndef LineBreakIteratorLatin1_D_HPP
#define LineBreakIteratorLatin1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef struct LineBreakIteratorLatin1 LineBreakIteratorLatin1;
} // namespace capi
} // namespace

class LineBreakIteratorLatin1 {
public:

  inline int32_t next();

  inline const diplomat::capi::LineBreakIteratorLatin1* AsFFI() const;
  inline diplomat::capi::LineBreakIteratorLatin1* AsFFI();
  inline static const LineBreakIteratorLatin1* FromFFI(const diplomat::capi::LineBreakIteratorLatin1* ptr);
  inline static LineBreakIteratorLatin1* FromFFI(diplomat::capi::LineBreakIteratorLatin1* ptr);
  inline static void operator delete(void* ptr);
private:
  LineBreakIteratorLatin1() = delete;
  LineBreakIteratorLatin1(const LineBreakIteratorLatin1&) = delete;
  LineBreakIteratorLatin1(LineBreakIteratorLatin1&&) noexcept = delete;
  LineBreakIteratorLatin1 operator=(const LineBreakIteratorLatin1&) = delete;
  LineBreakIteratorLatin1 operator=(LineBreakIteratorLatin1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LineBreakIteratorLatin1_D_HPP
