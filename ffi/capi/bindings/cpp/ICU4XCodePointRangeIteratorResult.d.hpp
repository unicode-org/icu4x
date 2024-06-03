#ifndef ICU4XCodePointRangeIteratorResult_D_HPP
#define ICU4XCodePointRangeIteratorResult_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCodePointRangeIteratorResult.d.h"


struct ICU4XCodePointRangeIteratorResult {
  uint32_t start;
  uint32_t end;
  bool done;

  inline capi::ICU4XCodePointRangeIteratorResult AsFFI() const;
  inline static ICU4XCodePointRangeIteratorResult FromFFI(capi::ICU4XCodePointRangeIteratorResult c_struct);
};


#endif // ICU4XCodePointRangeIteratorResult_D_HPP
