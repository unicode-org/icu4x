#ifndef CodePointRangeIterator_HPP
#define CodePointRangeIterator_HPP

#include "CodePointRangeIterator.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "CodePointRangeIterator.h"
#include "CodePointRangeIteratorResult.hpp"


inline CodePointRangeIteratorResult CodePointRangeIterator::next() {
  auto result = capi::CodePointRangeIterator_next(this->AsFFI());
  return CodePointRangeIteratorResult::FromFFI(result);
}

inline const capi::CodePointRangeIterator* CodePointRangeIterator::AsFFI() const {
  return reinterpret_cast<const capi::CodePointRangeIterator*>(this);
}

inline capi::CodePointRangeIterator* CodePointRangeIterator::AsFFI() {
  return reinterpret_cast<capi::CodePointRangeIterator*>(this);
}

inline const CodePointRangeIterator* CodePointRangeIterator::FromFFI(const capi::CodePointRangeIterator* ptr) {
  return reinterpret_cast<const CodePointRangeIterator*>(ptr);
}

inline CodePointRangeIterator* CodePointRangeIterator::FromFFI(capi::CodePointRangeIterator* ptr) {
  return reinterpret_cast<CodePointRangeIterator*>(ptr);
}

inline void CodePointRangeIterator::operator delete(void* ptr) {
  capi::CodePointRangeIterator_destroy(reinterpret_cast<capi::CodePointRangeIterator*>(ptr));
}


#endif // CodePointRangeIterator_HPP
