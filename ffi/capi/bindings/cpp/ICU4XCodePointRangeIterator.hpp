#ifndef ICU4XCodePointRangeIterator_HPP
#define ICU4XCodePointRangeIterator_HPP

#include "ICU4XCodePointRangeIterator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointRangeIterator.h"
#include "ICU4XCodePointRangeIteratorResult.hpp"


inline ICU4XCodePointRangeIteratorResult ICU4XCodePointRangeIterator::next() {
  auto result = capi::ICU4XCodePointRangeIterator_next(this->AsFFI());
  return ICU4XCodePointRangeIteratorResult::FromFFI(result);
}

inline const capi::ICU4XCodePointRangeIterator* ICU4XCodePointRangeIterator::AsFFI() const {
  return reinterpret_cast<const capi::ICU4XCodePointRangeIterator*>(this);
}

inline capi::ICU4XCodePointRangeIterator* ICU4XCodePointRangeIterator::AsFFI() {
  return reinterpret_cast<capi::ICU4XCodePointRangeIterator*>(this);
}

inline const ICU4XCodePointRangeIterator* ICU4XCodePointRangeIterator::FromFFI(const capi::ICU4XCodePointRangeIterator* ptr) {
  return reinterpret_cast<const ICU4XCodePointRangeIterator*>(ptr);
}

inline ICU4XCodePointRangeIterator* ICU4XCodePointRangeIterator::FromFFI(capi::ICU4XCodePointRangeIterator* ptr) {
  return reinterpret_cast<ICU4XCodePointRangeIterator*>(ptr);
}

inline void ICU4XCodePointRangeIterator::operator delete(void* ptr) {
  capi::ICU4XCodePointRangeIterator_destroy(reinterpret_cast<capi::ICU4XCodePointRangeIterator*>(ptr));
}


#endif // ICU4XCodePointRangeIterator_HPP
