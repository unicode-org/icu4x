#ifndef ICU4XWordBreakIteratorUtf16_HPP
#define ICU4XWordBreakIteratorUtf16_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XWordBreakIteratorUtf16.h"

#include "ICU4XSegmenterRuleStatusType.hpp"

/**
 * A destruction policy for using ICU4XWordBreakIteratorUtf16 with std::unique_ptr.
 */
struct ICU4XWordBreakIteratorUtf16Deleter {
  void operator()(capi::ICU4XWordBreakIteratorUtf16* l) const noexcept {
    capi::ICU4XWordBreakIteratorUtf16_destroy(l);
  }
};

/**
 * 
 * 
 * See the [Rust documentation for `WordBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html) for more information.
 */
class ICU4XWordBreakIteratorUtf16 {
 public:

  /**
   * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
   * out of range of a 32-bit signed integer.
   * 
   * See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.next) for more information.
   */
  int32_t next();

  /**
   * Return the status value of break boundary.
   * 
   * See the [Rust documentation for `rule_status`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.rule_status) for more information.
   */
  ICU4XSegmenterRuleStatusType rule_status() const;

  /**
   * Return true when break boundary is word-like such as letter/number/CJK
   * 
   * See the [Rust documentation for `is_word_like`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.is_word_like) for more information.
   */
  bool is_word_like() const;
  inline const capi::ICU4XWordBreakIteratorUtf16* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XWordBreakIteratorUtf16* AsFFIMut() { return this->inner.get(); }
  inline ICU4XWordBreakIteratorUtf16(capi::ICU4XWordBreakIteratorUtf16* i) : inner(i) {}
  ICU4XWordBreakIteratorUtf16() = default;
  ICU4XWordBreakIteratorUtf16(ICU4XWordBreakIteratorUtf16&&) noexcept = default;
  ICU4XWordBreakIteratorUtf16& operator=(ICU4XWordBreakIteratorUtf16&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XWordBreakIteratorUtf16, ICU4XWordBreakIteratorUtf16Deleter> inner;
};


inline int32_t ICU4XWordBreakIteratorUtf16::next() {
  return capi::ICU4XWordBreakIteratorUtf16_next(this->inner.get());
}
inline ICU4XSegmenterRuleStatusType ICU4XWordBreakIteratorUtf16::rule_status() const {
  return static_cast<ICU4XSegmenterRuleStatusType>(capi::ICU4XWordBreakIteratorUtf16_rule_status(this->inner.get()));
}
inline bool ICU4XWordBreakIteratorUtf16::is_word_like() const {
  return capi::ICU4XWordBreakIteratorUtf16_is_word_like(this->inner.get());
}
#endif
