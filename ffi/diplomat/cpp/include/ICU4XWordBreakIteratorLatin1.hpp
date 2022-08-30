#ifndef ICU4XWordBreakIteratorLatin1_HPP
#define ICU4XWordBreakIteratorLatin1_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XWordBreakIteratorLatin1.h"


/**
 * A destruction policy for using ICU4XWordBreakIteratorLatin1 with std::unique_ptr.
 */
struct ICU4XWordBreakIteratorLatin1Deleter {
  void operator()(capi::ICU4XWordBreakIteratorLatin1* l) const noexcept {
    capi::ICU4XWordBreakIteratorLatin1_destroy(l);
  }
};
class ICU4XWordBreakIteratorLatin1 {
 public:

  /**
   * Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
   * out of range of a 32-bit signed integer.
   */
  int32_t next();
  inline const capi::ICU4XWordBreakIteratorLatin1* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XWordBreakIteratorLatin1* AsFFIMut() { return this->inner.get(); }
  inline ICU4XWordBreakIteratorLatin1(capi::ICU4XWordBreakIteratorLatin1* i) : inner(i) {}
  ICU4XWordBreakIteratorLatin1() = default;
  ICU4XWordBreakIteratorLatin1(ICU4XWordBreakIteratorLatin1&&) noexcept = default;
  ICU4XWordBreakIteratorLatin1& operator=(ICU4XWordBreakIteratorLatin1&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XWordBreakIteratorLatin1, ICU4XWordBreakIteratorLatin1Deleter> inner;
};


inline int32_t ICU4XWordBreakIteratorLatin1::next() {
  return capi::ICU4XWordBreakIteratorLatin1_next(this->inner.get());
}
#endif
