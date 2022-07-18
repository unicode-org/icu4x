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


/**
 * A destruction policy for using ICU4XWordBreakIteratorUtf16 with std::unique_ptr.
 */
struct ICU4XWordBreakIteratorUtf16Deleter {
  void operator()(capi::ICU4XWordBreakIteratorUtf16* l) const noexcept {
    capi::ICU4XWordBreakIteratorUtf16_destroy(l);
  }
};
class ICU4XWordBreakIteratorUtf16 {
 public:

  /**
   * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
   * out of range of a 32-bit signed integer.
   */
  int32_t next();
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
#endif
