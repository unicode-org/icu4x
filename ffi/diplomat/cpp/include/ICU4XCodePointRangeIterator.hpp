#ifndef ICU4XCodePointRangeIterator_HPP
#define ICU4XCodePointRangeIterator_HPP
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <algorithm>
#include <memory>
#include <variant>
#include <optional>
#include "diplomat_runtime.hpp"

#include "ICU4XCodePointRangeIterator.h"

struct ICU4XCodePointRangeIteratorResult;

/**
 * A destruction policy for using ICU4XCodePointRangeIterator with std::unique_ptr.
 */
struct ICU4XCodePointRangeIteratorDeleter {
  void operator()(capi::ICU4XCodePointRangeIterator* l) const noexcept {
    capi::ICU4XCodePointRangeIterator_destroy(l);
  }
};

/**
 * An iterator over code point ranges, produced by `ICU4XCodePointSetData` or
 * one of the `ICU4XCodePointMapData` types
 */
class ICU4XCodePointRangeIterator {
 public:

  /**
   * Advance the iterator by one and return the next range.
   * 
   * If the iterator is out of items, `done` will be true
   */
  ICU4XCodePointRangeIteratorResult next();
  inline const capi::ICU4XCodePointRangeIterator* AsFFI() const { return this->inner.get(); }
  inline capi::ICU4XCodePointRangeIterator* AsFFIMut() { return this->inner.get(); }
  inline ICU4XCodePointRangeIterator(capi::ICU4XCodePointRangeIterator* i) : inner(i) {}
  ICU4XCodePointRangeIterator() = default;
  ICU4XCodePointRangeIterator(ICU4XCodePointRangeIterator&&) noexcept = default;
  ICU4XCodePointRangeIterator& operator=(ICU4XCodePointRangeIterator&& other) noexcept = default;
 private:
  std::unique_ptr<capi::ICU4XCodePointRangeIterator, ICU4XCodePointRangeIteratorDeleter> inner;
};

#include "ICU4XCodePointRangeIteratorResult.hpp"

inline ICU4XCodePointRangeIteratorResult ICU4XCodePointRangeIterator::next() {
  capi::ICU4XCodePointRangeIteratorResult diplomat_raw_struct_out_value = capi::ICU4XCodePointRangeIterator_next(this->inner.get());
  return ICU4XCodePointRangeIteratorResult{ .start = std::move(diplomat_raw_struct_out_value.start), .end = std::move(diplomat_raw_struct_out_value.end), .done = std::move(diplomat_raw_struct_out_value.done) };
}
#endif
