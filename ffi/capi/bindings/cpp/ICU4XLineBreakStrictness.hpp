#ifndef ICU4XLineBreakStrictness_HPP
#define ICU4XLineBreakStrictness_HPP

#include "ICU4XLineBreakStrictness.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLineBreakStrictness.h"


inline capi::ICU4XLineBreakStrictness ICU4XLineBreakStrictness::AsFFI() const {
  return static_cast<capi::ICU4XLineBreakStrictness>(value);
}

inline ICU4XLineBreakStrictness ICU4XLineBreakStrictness::FromFFI(capi::ICU4XLineBreakStrictness c_enum) {
  switch (c_enum) {
    case capi::ICU4XLineBreakStrictness_Loose:
    case capi::ICU4XLineBreakStrictness_Normal:
    case capi::ICU4XLineBreakStrictness_Strict:
    case capi::ICU4XLineBreakStrictness_Anywhere:
      return static_cast<ICU4XLineBreakStrictness::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XLineBreakStrictness_HPP
