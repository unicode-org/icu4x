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
class ICU4XLineBreakIterator;

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
   * Construct an [`ICU4XLineBreakSegmenter`] from an locale identifier.
   * See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_segmenter/struct.LineBreakSegmenter.html#method.from_bytes) for more information.
   */
  static diplomat::result<ICU4XLineBreakSegmenter, std::monostate> try_new();
  ICU4XLineBreakIterator segment_str(const std::string_view input) const;
  inline const capi::ICU4XLineBreakSegmenter* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLineBreakSegmenter* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLineBreakSegmenter(capi::ICU4XLineBreakSegmenter* i) : inner(i) {}
  ICU4XLineBreakSegmenter() = default;
  ICU4XLineBreakSegmenter(ICU4XLineBreakSegmenter&&) noexcept = default;
  ICU4XLineBreakSegmenter& operator=(ICU4XLineBreakSegmenter&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLineBreakSegmenter, ICU4XLineBreakSegmenterDeleter> inner;
};

#include "ICU4XLineBreakIterator.hpp"

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
inline ICU4XLineBreakIterator ICU4XLineBreakSegmenter::segment_str(const std::string_view input) const {
  return ICU4XLineBreakIterator(capi::ICU4XLineBreakSegmenter_segment_str(this->inner.get(), input.data(), input.size()));
}
#endif
