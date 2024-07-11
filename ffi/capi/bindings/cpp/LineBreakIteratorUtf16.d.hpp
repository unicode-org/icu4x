#ifndef LineBreakIteratorUtf16_D_HPP
#define LineBreakIteratorUtf16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct LineBreakIteratorUtf16 LineBreakIteratorUtf16;
}

class LineBreakIteratorUtf16 {
public:

  inline int32_t next();

  inline const capi::LineBreakIteratorUtf16* AsFFI() const;
  inline capi::LineBreakIteratorUtf16* AsFFI();
  inline static const LineBreakIteratorUtf16* FromFFI(const capi::LineBreakIteratorUtf16* ptr);
  inline static LineBreakIteratorUtf16* FromFFI(capi::LineBreakIteratorUtf16* ptr);
  inline static void operator delete(void* ptr);
private:
  LineBreakIteratorUtf16() = delete;
  LineBreakIteratorUtf16(const LineBreakIteratorUtf16&) = delete;
  LineBreakIteratorUtf16(LineBreakIteratorUtf16&&) noexcept = delete;
  LineBreakIteratorUtf16 operator=(const LineBreakIteratorUtf16&) = delete;
  LineBreakIteratorUtf16 operator=(LineBreakIteratorUtf16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LineBreakIteratorUtf16_D_HPP
