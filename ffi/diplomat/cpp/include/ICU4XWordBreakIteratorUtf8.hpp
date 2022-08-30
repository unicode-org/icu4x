#ifndef ICU4XWordBreakIteratorUtf8_HPP
#define ICU4XWordBreakIteratorUtf8_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XWordBreakIteratorUtf8.h"


/**
 * A destruction policy for using ICU4XWordBreakIteratorUtf8 with std::unique_ptr.
 */
struct ICU4XWordBreakIteratorUtf8Deleter {
  void operator()(capi::ICU4XWordBreakIteratorUtf8* l) const noexcept {
    capi::ICU4XWordBreakIteratorUtf8_destroy(l);
  }
};
class ICU4XWordBreakIteratorUtf8 {
 public:

  /**
   * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
   * out of range of a 32-bit signed integer.
   */
  int32_t next();
  inline const capi::ICU4XWordBreakIteratorUtf8* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XWordBreakIteratorUtf8* AsFFIMut() { return this->inner.get(); }
  inline ICU4XWordBreakIteratorUtf8(capi::ICU4XWordBreakIteratorUtf8* i) : inner(i) {}
  ICU4XWordBreakIteratorUtf8() = default;
  ICU4XWordBreakIteratorUtf8(ICU4XWordBreakIteratorUtf8&&) noexcept = default;
  ICU4XWordBreakIteratorUtf8& operator=(ICU4XWordBreakIteratorUtf8&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XWordBreakIteratorUtf8, ICU4XWordBreakIteratorUtf8Deleter> inner;
};


inline int32_t ICU4XWordBreakIteratorUtf8::next() {
  return capi::ICU4XWordBreakIteratorUtf8_next(this->inner.get());
}
#endif
