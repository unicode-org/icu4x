#ifndef icu4x_GraphemeClusterBreakIteratorUtf16_D_HPP
#define icu4x_GraphemeClusterBreakIteratorUtf16_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    struct GraphemeClusterBreakIteratorUtf16;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `GraphemeClusterBreakIterator`](https://docs.rs/icu/2.0.0/icu/segmenter/iterators/struct.GraphemeClusterBreakIterator.html) for more information.
 */
class GraphemeClusterBreakIteratorUtf16 {
public:

  /**
   * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
   * out of range of a 32-bit signed integer.
   *
   * See the [Rust documentation for `next`](https://docs.rs/icu/2.0.0/icu/segmenter/iterators/struct.GraphemeClusterBreakIterator.html#method.next) for more information.
   */
  inline int32_t next();

  inline const icu4x::capi::GraphemeClusterBreakIteratorUtf16* AsFFI() const;
  inline icu4x::capi::GraphemeClusterBreakIteratorUtf16* AsFFI();
  inline static const icu4x::GraphemeClusterBreakIteratorUtf16* FromFFI(const icu4x::capi::GraphemeClusterBreakIteratorUtf16* ptr);
  inline static icu4x::GraphemeClusterBreakIteratorUtf16* FromFFI(icu4x::capi::GraphemeClusterBreakIteratorUtf16* ptr);
  inline static void operator delete(void* ptr);
private:
  GraphemeClusterBreakIteratorUtf16() = delete;
  GraphemeClusterBreakIteratorUtf16(const icu4x::GraphemeClusterBreakIteratorUtf16&) = delete;
  GraphemeClusterBreakIteratorUtf16(icu4x::GraphemeClusterBreakIteratorUtf16&&) noexcept = delete;
  GraphemeClusterBreakIteratorUtf16 operator=(const icu4x::GraphemeClusterBreakIteratorUtf16&) = delete;
  GraphemeClusterBreakIteratorUtf16 operator=(icu4x::GraphemeClusterBreakIteratorUtf16&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};

} // namespace
#endif // icu4x_GraphemeClusterBreakIteratorUtf16_D_HPP
