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


namespace capi {
    extern "C" {
    
    CodePointRangeIteratorResult ICU4XCodePointRangeIterator_next(CodePointRangeIterator* self);
    
    
    void ICU4XCodePointRangeIterator_destroy(CodePointRangeIterator* self);
    
    } // extern "C"
}

inline CodePointRangeIteratorResult CodePointRangeIterator::next() {
  auto result = capi::ICU4XCodePointRangeIterator_next(this->AsFFI());
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
  capi::ICU4XCodePointRangeIterator_destroy(reinterpret_cast<capi::CodePointRangeIterator*>(ptr));
}


#endif // CodePointRangeIterator_HPP
