#ifndef GraphemeClusterBreakIteratorUtf16_D_HPP
#define GraphemeClusterBreakIteratorUtf16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef struct GraphemeClusterBreakIteratorUtf16 GraphemeClusterBreakIteratorUtf16;
} // namespace capi
} // namespace

class GraphemeClusterBreakIteratorUtf16 {
public:

  inline int32_t next();

  inline const diplomat::capi::GraphemeClusterBreakIteratorUtf16* AsFFI() const;
  inline diplomat::capi::GraphemeClusterBreakIteratorUtf16* AsFFI();
  inline static const GraphemeClusterBreakIteratorUtf16* FromFFI(const diplomat::capi::GraphemeClusterBreakIteratorUtf16* ptr);
  inline static GraphemeClusterBreakIteratorUtf16* FromFFI(diplomat::capi::GraphemeClusterBreakIteratorUtf16* ptr);
  inline static void operator delete(void* ptr);
private:
  GraphemeClusterBreakIteratorUtf16() = delete;
  GraphemeClusterBreakIteratorUtf16(const GraphemeClusterBreakIteratorUtf16&) = delete;
  GraphemeClusterBreakIteratorUtf16(GraphemeClusterBreakIteratorUtf16&&) noexcept = delete;
  GraphemeClusterBreakIteratorUtf16 operator=(const GraphemeClusterBreakIteratorUtf16&) = delete;
  GraphemeClusterBreakIteratorUtf16 operator=(GraphemeClusterBreakIteratorUtf16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // GraphemeClusterBreakIteratorUtf16_D_HPP
