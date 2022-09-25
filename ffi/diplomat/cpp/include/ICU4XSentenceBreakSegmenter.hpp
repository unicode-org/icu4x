#ifndef ICU4XSentenceBreakSegmenter_HPP
#define ICU4XSentenceBreakSegmenter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XSentenceBreakSegmenter.h"

class ICU4XDataProvider;
class ICU4XSentenceBreakSegmenter;
#include "ICU4XError.hpp"
class ICU4XSentenceBreakIteratorUtf8;
class ICU4XSentenceBreakIteratorUtf16;
class ICU4XSentenceBreakIteratorLatin1;

/**
 * A destruction policy for using ICU4XSentenceBreakSegmenter with std::unique_ptr.
 */
struct ICU4XSentenceBreakSegmenterDeleter {
  void operator()(capi::ICU4XSentenceBreakSegmenter* l) const noexcept {
    capi::ICU4XSentenceBreakSegmenter_destroy(l);
  }
};

/**
 * An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.
 * 
 * See the [Rust documentation for `SentenceBreakSegmenter`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceBreakSegmenter.html) for more information.
 */
class ICU4XSentenceBreakSegmenter {
 public:

  /**
   * Construct an [`ICU4XSentenceBreakSegmenter`].
   * 
   * See the [Rust documentation for `try_new_unstable`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceBreakSegmenter.html#method.try_new_unstable) for more information.
   */
  static diplomat::result<ICU4XSentenceBreakSegmenter, ICU4XError> create(const ICU4XDataProvider& provider);

  /**
   * Segments a (potentially ill-formed) UTF-8 string.
   * 
   * See the [Rust documentation for `segment_utf8`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceBreakSegmenter.html#method.segment_utf8) for more information.
   */
  ICU4XSentenceBreakIteratorUtf8 segment_utf8(const std::string_view input) const;

  /**
   * Segments a UTF-16 string.
   * 
   * See the [Rust documentation for `segment_utf16`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceBreakSegmenter.html#method.segment_utf16) for more information.
   */
  ICU4XSentenceBreakIteratorUtf16 segment_utf16(const diplomat::span<uint16_t> input) const;

  /**
   * Segments a Latin-1 string.
   * 
   * See the [Rust documentation for `segment_latin1`](https://unicode-org.github.io/icu4x-docs/doc/icu/segmenter/struct.SentenceBreakSegmenter.html#method.segment_latin1) for more information.
   */
  ICU4XSentenceBreakIteratorLatin1 segment_latin1(const diplomat::span<uint8_t> input) const;
  inline const capi::ICU4XSentenceBreakSegmenter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XSentenceBreakSegmenter* AsFFIMut() { return this->inner.get(); }
  inline ICU4XSentenceBreakSegmenter(capi::ICU4XSentenceBreakSegmenter* i) : inner(i) {}
  ICU4XSentenceBreakSegmenter() = default;
  ICU4XSentenceBreakSegmenter(ICU4XSentenceBreakSegmenter&&) noexcept = default;
  ICU4XSentenceBreakSegmenter& operator=(ICU4XSentenceBreakSegmenter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XSentenceBreakSegmenter, ICU4XSentenceBreakSegmenterDeleter> inner;
};

#include "ICU4XDataProvider.hpp"
#include "ICU4XSentenceBreakIteratorUtf8.hpp"
#include "ICU4XSentenceBreakIteratorUtf16.hpp"
#include "ICU4XSentenceBreakIteratorLatin1.hpp"

inline diplomat::result<ICU4XSentenceBreakSegmenter, ICU4XError> ICU4XSentenceBreakSegmenter::create(const ICU4XDataProvider& provider) {
  auto diplomat_result_raw_out_value = capi::ICU4XSentenceBreakSegmenter_create(provider.AsFFI());
  diplomat::result<ICU4XSentenceBreakSegmenter, ICU4XError> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok<ICU4XSentenceBreakSegmenter>(std::move(ICU4XSentenceBreakSegmenter(diplomat_result_raw_out_value.ok)));
  } else {
    diplomat_result_out_value = diplomat::Err<ICU4XError>(std::move(static_cast<ICU4XError>(diplomat_result_raw_out_value.err)));
  }
  return diplomat_result_out_value;
}
inline ICU4XSentenceBreakIteratorUtf8 ICU4XSentenceBreakSegmenter::segment_utf8(const std::string_view input) const {
  return ICU4XSentenceBreakIteratorUtf8(capi::ICU4XSentenceBreakSegmenter_segment_utf8(this->inner.get(), input.data(), input.size()));
}
inline ICU4XSentenceBreakIteratorUtf16 ICU4XSentenceBreakSegmenter::segment_utf16(const diplomat::span<uint16_t> input) const {
  return ICU4XSentenceBreakIteratorUtf16(capi::ICU4XSentenceBreakSegmenter_segment_utf16(this->inner.get(), input.data(), input.size()));
}
inline ICU4XSentenceBreakIteratorLatin1 ICU4XSentenceBreakSegmenter::segment_latin1(const diplomat::span<uint8_t> input) const {
  return ICU4XSentenceBreakIteratorLatin1(capi::ICU4XSentenceBreakSegmenter_segment_latin1(this->inner.get(), input.data(), input.size()));
}
#endif
