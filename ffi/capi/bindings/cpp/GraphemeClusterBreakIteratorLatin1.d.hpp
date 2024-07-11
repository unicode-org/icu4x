#ifndef GraphemeClusterBreakIteratorLatin1_D_HPP
#define GraphemeClusterBreakIteratorLatin1_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef struct GraphemeClusterBreakIteratorLatin1 GraphemeClusterBreakIteratorLatin1;
} // namespace capi
} // namespace

class GraphemeClusterBreakIteratorLatin1 {
public:

  inline int32_t next();

  inline const diplomat::capi::GraphemeClusterBreakIteratorLatin1* AsFFI() const;
  inline diplomat::capi::GraphemeClusterBreakIteratorLatin1* AsFFI();
  inline static const GraphemeClusterBreakIteratorLatin1* FromFFI(const diplomat::capi::GraphemeClusterBreakIteratorLatin1* ptr);
  inline static GraphemeClusterBreakIteratorLatin1* FromFFI(diplomat::capi::GraphemeClusterBreakIteratorLatin1* ptr);
  inline static void operator delete(void* ptr);
private:
  GraphemeClusterBreakIteratorLatin1() = delete;
  GraphemeClusterBreakIteratorLatin1(const GraphemeClusterBreakIteratorLatin1&) = delete;
  GraphemeClusterBreakIteratorLatin1(GraphemeClusterBreakIteratorLatin1&&) noexcept = delete;
  GraphemeClusterBreakIteratorLatin1 operator=(const GraphemeClusterBreakIteratorLatin1&) = delete;
  GraphemeClusterBreakIteratorLatin1 operator=(GraphemeClusterBreakIteratorLatin1&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // GraphemeClusterBreakIteratorLatin1_D_HPP
