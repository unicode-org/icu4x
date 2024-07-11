#ifndef GraphemeClusterBreakIteratorUtf8_D_HPP
#define GraphemeClusterBreakIteratorUtf8_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct GraphemeClusterBreakIteratorUtf8 GraphemeClusterBreakIteratorUtf8;
}

class GraphemeClusterBreakIteratorUtf8 {
public:

  inline int32_t next();

  inline const capi::GraphemeClusterBreakIteratorUtf8* AsFFI() const;
  inline capi::GraphemeClusterBreakIteratorUtf8* AsFFI();
  inline static const GraphemeClusterBreakIteratorUtf8* FromFFI(const capi::GraphemeClusterBreakIteratorUtf8* ptr);
  inline static GraphemeClusterBreakIteratorUtf8* FromFFI(capi::GraphemeClusterBreakIteratorUtf8* ptr);
  inline static void operator delete(void* ptr);
private:
  GraphemeClusterBreakIteratorUtf8() = delete;
  GraphemeClusterBreakIteratorUtf8(const GraphemeClusterBreakIteratorUtf8&) = delete;
  GraphemeClusterBreakIteratorUtf8(GraphemeClusterBreakIteratorUtf8&&) noexcept = delete;
  GraphemeClusterBreakIteratorUtf8 operator=(const GraphemeClusterBreakIteratorUtf8&) = delete;
  GraphemeClusterBreakIteratorUtf8 operator=(GraphemeClusterBreakIteratorUtf8&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // GraphemeClusterBreakIteratorUtf8_D_HPP
