#ifndef ICU4XLineBreakIterator_HPP
#define ICU4XLineBreakIterator_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

namespace capi {
#include "ICU4XLineBreakIterator.h"
}


/**
 * A destruction policy for using ICU4XLineBreakIterator with std::unique_ptr.
 */
struct ICU4XLineBreakIteratorDeleter {
  void operator()(capi::ICU4XLineBreakIterator* l) const noexcept {
    capi::ICU4XLineBreakIterator_destroy(l);
  }
};
class ICU4XLineBreakIterator {
 public:

  /**
   * Find the next breakpoint. Returns -1 if at the end of the string or if the index is
   * out of range of a 32-bit signed integer.
   */
  int32_t next();
  inline const capi::ICU4XLineBreakIterator* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XLineBreakIterator* AsFFIMut() { return this->inner.get(); }
  inline ICU4XLineBreakIterator(capi::ICU4XLineBreakIterator* i) : inner(i) {}
  ICU4XLineBreakIterator() = default;
  ICU4XLineBreakIterator(ICU4XLineBreakIterator&&) noexcept = default;
  ICU4XLineBreakIterator& operator=(ICU4XLineBreakIterator&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XLineBreakIterator, ICU4XLineBreakIteratorDeleter> inner;
};


inline int32_t ICU4XLineBreakIterator::next() {
  return capi::ICU4XLineBreakIterator_next(this->inner.get());
}
#endif
