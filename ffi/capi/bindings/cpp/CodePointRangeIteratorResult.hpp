#ifndef CodePointRangeIteratorResult_HPP
#define CodePointRangeIteratorResult_HPP

#include "CodePointRangeIteratorResult.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::CodePointRangeIteratorResult CodePointRangeIteratorResult::AsFFI() const {
  return diplomat::capi::CodePointRangeIteratorResult {
    .start = start,
    .end = end,
    .done = done,
  };
}

inline CodePointRangeIteratorResult CodePointRangeIteratorResult::FromFFI(diplomat::capi::CodePointRangeIteratorResult c_struct) {
  return CodePointRangeIteratorResult {
    .start = c_struct.start,
    .end = c_struct.end,
    .done = c_struct.done,
  };
}


#endif // CodePointRangeIteratorResult_HPP
