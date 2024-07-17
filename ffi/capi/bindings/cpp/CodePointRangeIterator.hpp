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
#include "CodePointRangeIteratorResult.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    diplomat::capi::CodePointRangeIteratorResult icu4x_CodePointRangeIterator_next_mv1(diplomat::capi::CodePointRangeIterator* self);
    
    
    void icu4x_CodePointRangeIterator_destroy_mv1(CodePointRangeIterator* self);
    
    } // extern "C"
} // namespace capi
} // namespace

inline CodePointRangeIteratorResult CodePointRangeIterator::next() {
  auto result = diplomat::capi::icu4x_CodePointRangeIterator_next_mv1(this->AsFFI());
  return CodePointRangeIteratorResult::FromFFI(result);
}

inline const diplomat::capi::CodePointRangeIterator* CodePointRangeIterator::AsFFI() const {
  return reinterpret_cast<const diplomat::capi::CodePointRangeIterator*>(this);
}

inline diplomat::capi::CodePointRangeIterator* CodePointRangeIterator::AsFFI() {
  return reinterpret_cast<diplomat::capi::CodePointRangeIterator*>(this);
}

inline const CodePointRangeIterator* CodePointRangeIterator::FromFFI(const diplomat::capi::CodePointRangeIterator* ptr) {
  return reinterpret_cast<const CodePointRangeIterator*>(ptr);
}

inline CodePointRangeIterator* CodePointRangeIterator::FromFFI(diplomat::capi::CodePointRangeIterator* ptr) {
  return reinterpret_cast<CodePointRangeIterator*>(ptr);
}

inline void CodePointRangeIterator::operator delete(void* ptr) {
  diplomat::capi::icu4x_CodePointRangeIterator_destroy_mv1(reinterpret_cast<diplomat::capi::CodePointRangeIterator*>(ptr));
}


#endif // CodePointRangeIterator_HPP
