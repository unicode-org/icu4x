#ifndef ICU4XWordBreakSegmenter_HPP
#define ICU4XWordBreakSegmenter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XWordBreakSegmenter.h"

class ICU4XDataProvider;
class ICU4XWordBreakSegmenter;
#include "ICU4XError.hpp"
class ICU4XWordBreakIteratorUtf8;
class ICU4XWordBreakIteratorUtf16;
class ICU4XWordBreakIteratorLatin1;

/**
 * A destruction policy for using ICU4XWordBreakSegmenter with std::unique_ptr.
 */
struct ICU4XWordBreakSegmenterDeleter {
  void operator()(capi::ICU4XWordBreakSegmenter* l) const noexcept {
    capi::ICU4XWordBreakSegmenter_destroy(l);
  }
};

/**
 * An ICU4X word-break segmenter, capable of finding word breakpoints in strings.
 * 
 * See the [Rust documentation for `WordBreakSegmenter`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordBreakSegmenter.html) for more information.
 */
class ICU4XWordBreakSegmenter {
 public:

  /**
   * Construct an [`ICU4XWordBreakSegmenter`].
   * 
   * See the [Rust documentation for `try_new_unstable`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordBreakSegmenter.html#method.try_new_unstable) for more information.
   */
  static diplomat::result<ICU4XWordBreakSegmenter, ICU4XError> create(const ICU4XDataProvider& provider);

  /**
   * Segments a (potentially ill-formed) UTF-8 string.
   * 
   * See the [Rust documentation for `segment_utf8`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordBreakSegmenter.html#method.segment_utf8) for more information.
   */
  ICU4XWordBreakIteratorUtf8 segment_utf8(const std::string_view input) const;

  /**
   * Segments a UTF-16 string.
   * 
   * See the [Rust documentation for `segment_utf16`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordBreakSegmenter.html#method.segment_utf16) for more information.
   */
  ICU4XWordBreakIteratorUtf16 segment_utf16(const diplomat::span<uint16_t> input) const;

  /**
   * Segments a Latin-1 string.
   * 
   * See the [Rust documentation for `segment_latin1`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.WordBreakSegmenter.html#method.segment_latin1) for more information.
   */
  ICU4XWordBreakIteratorLatin1 segment_latin1(const diplomat::span<uint8_t> input) const;
  inline const capi::ICU4XWordBreakSegmenter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XWordBreakSegmenter* AsFFIMut() { return this->inner.get(); }
  inline ICU4XWordBreakSegmenter(capi::ICU4XWordBreakSegmenter* i) : inner(i) {}
  ICU4XWordBreakSegmenter() = default;
  ICU4XWordBreakSegmenter(ICU4XWordBreakSegmenter&&) noexcept = default;
  ICU4XWordBreakSegmenter& operator=(ICU4XWordBreakSegmenter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XWordBreakSegmenter, ICU4XWordBreakSegmenterDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XWordBreakIteratorUtf8.hpp"
#include "ICU4XWordBreakIteratorUtf16.hpp"
#include "ICU4XWordBreakIteratorLatin1.hpp"

inline diplomat::result<ICU4XWordBreakSegmenter, ICU4XError> ICU4XWordBreakSegmenter::create(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XWordBreakSegmenter_create(provider.AsFFI());
  diplomat::result<ICU4XWordBreakSegmenter, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XWordBreakSegmenter>(std::move(ICU4XWordBreakSegmenter(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XWordBreakIteratorUtf8 ICU4XWordBreakSegmenter::segment_utf8(const std::string_view input) const {
  return ICU4XWordBreakIteratorUtf8(capi::ICU4XWordBreakSegmenter_segment_utf8(this->inner.get(), input.data(), input.size()));
}
inline ICU4XWordBreakIteratorUtf16 ICU4XWordBreakSegmenter::segment_utf16(const diplomat::span<uint16_t> input) const {
  return ICU4XWordBreakIteratorUtf16(capi::ICU4XWordBreakSegmenter_segment_utf16(this->inner.get(), input.data(), input.size()));
}
inline ICU4XWordBreakIteratorLatin1 ICU4XWordBreakSegmenter::segment_latin1(const diplomat::span<uint8_t> input) const {
  return ICU4XWordBreakIteratorLatin1(capi::ICU4XWordBreakSegmenter_segment_latin1(this->inner.get(), input.data(), input.size()));
}
#endif
