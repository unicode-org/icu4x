#ifndef ICU4XBidiParagraph_HPP
#define ICU4XBidiParagraph_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XBidiParagraph.h"
}

#include "ICU4XBidiDirection.hpp"

/**
 * A destruction policy for using ICU4XBidiParagraph with std::unique_ptr.
 */
struct ICU4XBidiParagraphDeleter {
  void operator()(capi::ICU4XBidiParagraph* l) const noexcept {
    capi::ICU4XBidiParagraph_destroy(l);
  }
};

/**
 * Bidi information for a single processed paragraph
 */
class ICU4XBidiParagraph {
 public:

  /**
   * The primary direction of this paragraph
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
   */
  ICU4XBidiDirection direction() const;

  /**
   * The number of bytes in this paragraph
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.ParagraphInfo.html#method.len) for more information.
   */
  size_t size() const;

  /**
   * The start index of this paragraph within the source text
   */
  size_t range_start() const;

  /**
   * The end index of this paragraph within the source text
   */
  size_t range_end() const;

  /**
   * Reorder a line based on display order. The ranges are specified relative to the source text and must be contained
   * within this paragraph's range.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
   */
  template<typename W> diplomat::result<std::monostate, std::monostate> reorder_line_to_writeable(size_t range_start, size_t range_end, W& out) const;

  /**
   * Reorder a line based on display order. The ranges are specified relative to the source text and must be contained
   * within this paragraph's range.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
   */
  diplomat::result<std::string, std::monostate> reorder_line(size_t range_start, size_t range_end) const;

  /**
   * Get the BIDI level. This integer is conceptually a `unicode_bidi::Level`,
   * and can be further inspected using the static methods on this class.
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
   */
  uint8_t level_at(size_t pos) const;

  /**
   * Check if a Level returned by level_at is an RTL level.
   * 
   * Invalid levels (numbers greater than 125) will be assumed LTR
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_rtl) for more information.
   */
  static bool level_is_rtl(uint8_t level);

  /**
   * Check if a Level returned by level_at is an LTR level.
   * 
   * Invalid levels (numbers greater than 125) will be assumed LTR
   * 
   * See the [Rust documentation](https://unicode-org.github.io/icu4x-docs/doc/unicode_bidi/struct.Level.html#method.is_ltr) for more information.
   */
  static bool level_is_ltr(uint8_t level);
  inline const capi::ICU4XBidiParagraph* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XBidiParagraph* AsFFIMut() { return this->inner.get(); }
  inline ICU4XBidiParagraph(capi::ICU4XBidiParagraph* i) : inner(i) {}
  ICU4XBidiParagraph() = default;
  ICU4XBidiParagraph(ICU4XBidiParagraph&&) noexcept = default;
  ICU4XBidiParagraph& operator=(ICU4XBidiParagraph&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XBidiParagraph, ICU4XBidiParagraphDeleter> inner;
};


inline ICU4XBidiDirection ICU4XBidiParagraph::direction() const {
  return static_cast<ICU4XBidiDirection>(capi::ICU4XBidiParagraph_direction(this->inner.get()));
}
inline size_t ICU4XBidiParagraph::size() const {
  return capi::ICU4XBidiParagraph_size(this->inner.get());
}
inline size_t ICU4XBidiParagraph::range_start() const {
  return capi::ICU4XBidiParagraph_range_start(this->inner.get());
}
inline size_t ICU4XBidiParagraph::range_end() const {
  return capi::ICU4XBidiParagraph_range_end(this->inner.get());
}
template<typename W> inline diplomat::result<std::monostate, std::monostate> ICU4XBidiParagraph::reorder_line_to_writeable(size_t range_start, size_t range_end, W& out) const {
  capi::DiplomatWriteable out_writer = diplomat::WriteableTrait<W>::Construct(out);
  auto diplomat_result_raw_out_value = capi::ICU4XBidiParagraph_reorder_line(this->inner.get(), range_start, range_end, &out_writer);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value;
}
inline diplomat::result<std::string, std::monostate> ICU4XBidiParagraph::reorder_line(size_t range_start, size_t range_end) const {
  std::string diplomat_writeable_string;
  capi::DiplomatWriteable diplomat_writeable_out = diplomat::WriteableFromString(diplomat_writeable_string);
  auto diplomat_result_raw_out_value = capi::ICU4XBidiParagraph_reorder_line(this->inner.get(), range_start, range_end, &diplomat_writeable_out);
  diplomat::result<std::monostate, std::monostate> diplomat_result_out_value;
  if (diplomat_result_raw_out_value.is_ok) {
    diplomat_result_out_value = diplomat::Ok(std::monostate());
  } else {
    diplomat_result_out_value = diplomat::Err(std::monostate());
  }
  return diplomat_result_out_value.replace_ok(std::move(diplomat_writeable_string));
}
inline uint8_t ICU4XBidiParagraph::level_at(size_t pos) const {
  return capi::ICU4XBidiParagraph_level_at(this->inner.get(), pos);
}
inline bool ICU4XBidiParagraph::level_is_rtl(uint8_t level) {
  return capi::ICU4XBidiParagraph_level_is_rtl(level);
}
inline bool ICU4XBidiParagraph::level_is_ltr(uint8_t level) {
  return capi::ICU4XBidiParagraph_level_is_ltr(level);
}
#endif
