#ifndef LineBreakIteratorUtf8_D_HPP
#define LineBreakIteratorUtf8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct LineBreakIteratorUtf8 LineBreakIteratorUtf8;
}

class LineBreakIteratorUtf8 {
public:

  inline int32_t next();

  inline const capi::LineBreakIteratorUtf8* AsFFI() const;
  inline capi::LineBreakIteratorUtf8* AsFFI();
  inline static const LineBreakIteratorUtf8* FromFFI(const capi::LineBreakIteratorUtf8* ptr);
  inline static LineBreakIteratorUtf8* FromFFI(capi::LineBreakIteratorUtf8* ptr);
  inline static void operator delete(void* ptr);
private:
  LineBreakIteratorUtf8() = delete;
  LineBreakIteratorUtf8(const LineBreakIteratorUtf8&) = delete;
  LineBreakIteratorUtf8(LineBreakIteratorUtf8&&) noexcept = delete;
  LineBreakIteratorUtf8 operator=(const LineBreakIteratorUtf8&) = delete;
  LineBreakIteratorUtf8 operator=(LineBreakIteratorUtf8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // LineBreakIteratorUtf8_D_HPP
