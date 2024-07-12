#ifndef GraphemeClusterSegmenter_D_HPP
#define GraphemeClusterSegmenter_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"

namespace diplomat::capi { struct DataProvider; }
class DataProvider;
namespace diplomat::capi { struct GraphemeClusterBreakIteratorLatin1; }
class GraphemeClusterBreakIteratorLatin1;
namespace diplomat::capi { struct GraphemeClusterBreakIteratorUtf16; }
class GraphemeClusterBreakIteratorUtf16;
namespace diplomat::capi { struct GraphemeClusterBreakIteratorUtf8; }
class GraphemeClusterBreakIteratorUtf8;
class DataError;


namespace diplomat {
namespace capi {
    struct GraphemeClusterSegmenter;
} // namespace capi
} // namespace

class GraphemeClusterSegmenter {
public:

  inline static diplomat::result<std::unique_ptr<GraphemeClusterSegmenter>, DataError> create(const DataProvider& provider);

  inline std::unique_ptr<GraphemeClusterBreakIteratorUtf8> segment_utf8(std::string_view input) const;

  inline std::unique_ptr<GraphemeClusterBreakIteratorUtf16> segment_utf16(std::u16string_view input) const;

  inline std::unique_ptr<GraphemeClusterBreakIteratorLatin1> segment_latin1(diplomat::span<const uint8_t> input) const;

  inline const diplomat::capi::GraphemeClusterSegmenter* AsFFI() const;
  inline diplomat::capi::GraphemeClusterSegmenter* AsFFI();
  inline static const GraphemeClusterSegmenter* FromFFI(const diplomat::capi::GraphemeClusterSegmenter* ptr);
  inline static GraphemeClusterSegmenter* FromFFI(diplomat::capi::GraphemeClusterSegmenter* ptr);
  inline static void operator delete(void* ptr);
private:
  GraphemeClusterSegmenter() = delete;
  GraphemeClusterSegmenter(const GraphemeClusterSegmenter&) = delete;
  GraphemeClusterSegmenter(GraphemeClusterSegmenter&&) noexcept = delete;
  GraphemeClusterSegmenter operator=(const GraphemeClusterSegmenter&) = delete;
  GraphemeClusterSegmenter operator=(GraphemeClusterSegmenter&&) noexcept = delete;
  static void operator delete[](void*, size_t) = delete;
};


#endif // GraphemeClusterSegmenter_D_HPP
