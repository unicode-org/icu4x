#ifndef ICU4XLineBreakSegmenter_HPP
#define ICU4XLineBreakSegmenter_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XLineBreakSegmenter.h"
}

class ICU4XLineBreakSegmenter;
struct ICU4XLineBreakOptions;
class ICU4XLineBreakIteratorUtf8;
class ICU4XLineBreakIteratorUtf16;
class ICU4XLineBreakIteratorLatin1;

/**
 * A destruction policy for using ICU4XLineBreakSegmenter with std::unique_ptr.
 */
struct ICU4XLineBreakSegmenterDeleter {
  void operator()(capi::ICU4XLineBreakSegmenter* l) const noexcept {
    capi::ICU4XLineBreakSegmenter_destroy(l);
  }
};
class ICU4XLineBreakSegmenter {
 public:

  /**
   * Construct a [`ICU4XLineBreakSegmenter`] with default options.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new) for more information.
   */
  static diplomat::result<ICU4XLineBreakSegmenter, std::monostate> try_new();

  /**
   * Construct a [`ICU4XLineBreakSegmenter`] with custom options.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.try_new_with_options) for more information.
   */
  static diplomat::result<ICU4XLineBreakSegmenter, std::monostate> try_new_with_options(ICU4XLineBreakOptions options);

  /**
   * Segments a UTF-8 string.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_str) for more information.
   */
  ICU4XLineBreakIteratorUtf8 segment_utf8(const std::string_view input) const;

  /**
   * Segments a UTF-16 string.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_utf16) for more information.
   */
  ICU4XLineBreakIteratorUtf16 segment_utf16(const diplomat::span<uint16_t> input) const;

  /**
   * Segments a Latin-1 string.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.segment_latin1) for more information.
   */
  ICU4XLineBreakIteratorLatin1 segment_latin1(const diplomat::span<uint8_t> input) const;
  inline const capi::ICU4XLineBreakSegmenter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLineBreakSegmenter* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLineBreakSegmenter(capi::ICU4XLineBreakSegmenter* i) : inner(i) {}
  ICU4XLineBreakSegmenter() = default;
  ICU4XLineBreakSegmenter(ICU4XLineBreakSegmenter&&) noexcept = default;
  ICU4XLineBreakSegmenter& operator=(ICU4XLineBreakSegmenter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLineBreakSegmenter, ICU4XLineBreakSegmenterDeleter> inner;
};

#include "ICU4XLineBreakOptions.hpp"
#include "ICU4XLineBreakIteratorUtf8.hpp"
#include "ICU4XLineBreakIteratorUtf16.hpp"
#include "ICU4XLineBreakIteratorLatin1.hpp"

inline diplomat::result<ICU4XLineBreakSegmenter, std::monostate> ICU4XLineBreakSegmenter::try_new() {
  auto diplomat_result_raw_out_value = capi::ICU4XLineBreakSegmenter_try_new();
  diplomat::result<ICU4XLineBreakSegmenter, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XLineBreakSegmenter(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<ICU4XLineBreakSegmenter, std::monostate> ICU4XLineBreakSegmenter::try_new_with_options(ICU4XLineBreakOptions options) {
  ICU4XLineBreakOptions diplomat_wrapped_struct_options = options;
  auto diplomat_result_raw_out_value = capi::ICU4XLineBreakSegmenter_try_new_with_options(capi::ICU4XLineBreakOptions{ .line_break_rule = static_cast<capi::ICU4XLineBreakRule>(diplomat_wrapped_struct_options.line_break_rule), .word_break_rule = static_cast<capi::ICU4XWordBreakRule>(diplomat_wrapped_struct_options.word_break_rule), .ja_zh = diplomat_wrapped_struct_options.ja_zh });
  diplomat::result<ICU4XLineBreakSegmenter, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(ICU4XLineBreakSegmenter(diplomat_result_raw_out_value.ok));
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline ICU4XLineBreakIteratorUtf8 ICU4XLineBreakSegmenter::segment_utf8(const std::string_view input) const {
  return ICU4XLineBreakIteratorUtf8(capi::ICU4XLineBreakSegmenter_segment_utf8(this->inner.get(), input.data(), input.size()));
}
inline ICU4XLineBreakIteratorUtf16 ICU4XLineBreakSegmenter::segment_utf16(const diplomat::span<uint16_t> input) const {
  return ICU4XLineBreakIteratorUtf16(capi::ICU4XLineBreakSegmenter_segment_utf16(this->inner.get(), input.data(), input.size()));
}
inline ICU4XLineBreakIteratorLatin1 ICU4XLineBreakSegmenter::segment_latin1(const diplomat::span<uint8_t> input) const {
  return ICU4XLineBreakIteratorLatin1(capi::ICU4XLineBreakSegmenter_segment_latin1(this->inner.get(), input.data(), input.size()));
}
#endif
