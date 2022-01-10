#ifndef ICU4XLineBreakIteratorUtf8_HPP
#define ICU4XLineBreakIteratorUtf8_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XLineBreakIteratorUtf8.h"
}


/**
 * A destruction policy for using ICU4XLineBreakIteratorUtf8 with std::unique_ptr.
 */
struct ICU4XLineBreakIteratorUtf8Deleter {
  void operator()(capi::ICU4XLineBreakIteratorUtf8* l) const noexcept {
    capi::ICU4XLineBreakIteratorUtf8_destroy(l);
  }
};
class ICU4XLineBreakIteratorUtf8 {
 public:

  /**
   * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
   * out of range of a 32-bit signed integer.
   */
  int32_t next();
  inline const capi::ICU4XLineBreakIteratorUtf8* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLineBreakIteratorUtf8* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLineBreakIteratorUtf8(capi::ICU4XLineBreakIteratorUtf8* i) : inner(i) {}
  ICU4XLineBreakIteratorUtf8() = default;
  ICU4XLineBreakIteratorUtf8(ICU4XLineBreakIteratorUtf8&&) noexcept = default;
  ICU4XLineBreakIteratorUtf8& operator=(ICU4XLineBreakIteratorUtf8&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLineBreakIteratorUtf8, ICU4XLineBreakIteratorUtf8Deleter> inner;
};


inline int32_t ICU4XLineBreakIteratorUtf8::next() {
  return capi::ICU4XLineBreakIteratorUtf8_next(this->inner.get());
}
#endif
