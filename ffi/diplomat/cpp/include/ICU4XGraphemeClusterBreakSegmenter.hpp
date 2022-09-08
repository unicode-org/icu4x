#ifndef ICU4XGraphemeClusterBreakSegmenter_HPP
#define ICU4XGraphemeClusterBreakSegmenter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XGraphemeClusterBreakSegmenter.h"

class ICU4XDataProvider;
class ICU4XGraphemeClusterBreakSegmenter;
#include "ICU4XError.hpp"
class ICU4XGraphemeClusterBreakIteratorUtf8;
class ICU4XGraphemeClusterBreakIteratorUtf16;
class ICU4XGraphemeClusterBreakIteratorLatin1;

/**
 * A destruction policy for using ICU4XGraphemeClusterBreakSegmenter with std::unique_ptr.
 */
struct ICU4XGraphemeClusterBreakSegmenterDeleter {
  void operator()(capi::ICU4XGraphemeClusterBreakSegmenter* l) const noexcept {
    capi::ICU4XGraphemeClusterBreakSegmenter_destroy(l);
  }
};

/**
 * An ICU4X grapheme-cluster-break segmenter, capable of finding grapheme cluster breakpoints
 * in strings.
 * 
 * See the [Rust documentation for `GraphemeClusterBreakSegmenter`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html) for more information.
 */
class ICU4XGraphemeClusterBreakSegmenter {
 public:

  /**
   * Construct an [`ICU4XGraphemeClusterBreakSegmenter`].
   * 
   * See the [Rust documentation for `try_new`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XGraphemeClusterBreakSegmenter, ICU4XError> try_new(const ICU4XDataProvider& provider);

  /**
   * Segments a UTF-8 string.
   * 
   * See the [Rust documentation for `segment_str`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_str) for more information.
   */
  ICU4XGraphemeClusterBreakIteratorUtf8 segment_utf8(const std::string_view input) const;

  /**
   * Segments a UTF-16 string.
   * 
   * See the [Rust documentation for `segment_utf16`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_utf16) for more information.
   */
  ICU4XGraphemeClusterBreakIteratorUtf16 segment_utf16(const diplomat::span<uint16_t> input) const;

  /**
   * Segments a Latin-1 string.
   * 
   * See the [Rust documentation for `segment_latin1`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.GraphemeClusterBreakSegmenter.html#method.segment_latin1) for more information.
   */
  ICU4XGraphemeClusterBreakIteratorLatin1 segment_latin1(const diplomat::span<uint8_t> input) const;
  inline const capi::ICU4XGraphemeClusterBreakSegmenter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XGraphemeClusterBreakSegmenter* AsFFIMut() { return this->inner.get(); }
  inline ICU4XGraphemeClusterBreakSegmenter(capi::ICU4XGraphemeClusterBreakSegmenter* i) : inner(i) {}
  ICU4XGraphemeClusterBreakSegmenter() = default;
  ICU4XGraphemeClusterBreakSegmenter(ICU4XGraphemeClusterBreakSegmenter&&) noexcept = default;
  ICU4XGraphemeClusterBreakSegmenter& operator=(ICU4XGraphemeClusterBreakSegmenter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XGraphemeClusterBreakSegmenter, ICU4XGraphemeClusterBreakSegmenterDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XGraphemeClusterBreakIteratorUtf8.hpp"
#include "ICU4XGraphemeClusterBreakIteratorUtf16.hpp"
#include "ICU4XGraphemeClusterBreakIteratorLatin1.hpp"

inline diplomat::result<ICU4XGraphemeClusterBreakSegmenter, ICU4XError> ICU4XGraphemeClusterBreakSegmenter::try_new(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XGraphemeClusterBreakSegmenter_try_new(provider.AsFFI());
  diplomat::result<ICU4XGraphemeClusterBreakSegmenter, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XGraphemeClusterBreakSegmenter>(std::move(ICU4XGraphemeClusterBreakSegmenter(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XGraphemeClusterBreakIteratorUtf8 ICU4XGraphemeClusterBreakSegmenter::segment_utf8(const std::string_view input) const {
  return ICU4XGraphemeClusterBreakIteratorUtf8(capi::ICU4XGraphemeClusterBreakSegmenter_segment_utf8(this->inner.get(), input.data(), input.size()));
}
inline ICU4XGraphemeClusterBreakIteratorUtf16 ICU4XGraphemeClusterBreakSegmenter::segment_utf16(const diplomat::span<uint16_t> input) const {
  return ICU4XGraphemeClusterBreakIteratorUtf16(capi::ICU4XGraphemeClusterBreakSegmenter_segment_utf16(this->inner.get(), input.data(), input.size()));
}
inline ICU4XGraphemeClusterBreakIteratorLatin1 ICU4XGraphemeClusterBreakSegmenter::segment_latin1(const diplomat::span<uint8_t> input) const {
  return ICU4XGraphemeClusterBreakIteratorLatin1(capi::ICU4XGraphemeClusterBreakSegmenter_segment_latin1(this->inner.get(), input.data(), input.size()));
}
#endif
