#ifndef ICU4XCodePointRangeIteratorResult_HPP
#define ICU4XCodePointRangeIteratorResult_HPP

#include "ICU4XCodePointRangeIteratorResult.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::ICU4XCodePointRangeIteratorResult ICU4XCodePointRangeIteratorResult::AsFFI() const {
  return capi::ICU4XCodePointRangeIteratorResult {
    .start = start,
    .end = end,
    .done = done,
  };
}

inline ICU4XCodePointRangeIteratorResult ICU4XCodePointRangeIteratorResult::FromFFI(capi::ICU4XCodePointRangeIteratorResult c_struct) {
  return ICU4XCodePointRangeIteratorResult {
    .start = c_struct.start,
    .end = c_struct.end,
    .done = c_struct.done,
  };
}


#endif // ICU4XCodePointRangeIteratorResult_HPP
