#ifndef ICU4XFixedDecimalRoundingMode_HPP
#define ICU4XFixedDecimalRoundingMode_HPP

#include "ICU4XFixedDecimalRoundingMode.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalRoundingMode.h"


inline capi::ICU4XFixedDecimalRoundingMode ICU4XFixedDecimalRoundingMode::AsFFI() const {
  return static_cast<capi::ICU4XFixedDecimalRoundingMode>(value);
}

inline ICU4XFixedDecimalRoundingMode ICU4XFixedDecimalRoundingMode::FromFFI(capi::ICU4XFixedDecimalRoundingMode c_enum) {
  switch (c_enum) {
    case capi::ICU4XFixedDecimalRoundingMode_Ceil:
    case capi::ICU4XFixedDecimalRoundingMode_Expand:
    case capi::ICU4XFixedDecimalRoundingMode_Floor:
    case capi::ICU4XFixedDecimalRoundingMode_Trunc:
    case capi::ICU4XFixedDecimalRoundingMode_HalfCeil:
    case capi::ICU4XFixedDecimalRoundingMode_HalfExpand:
    case capi::ICU4XFixedDecimalRoundingMode_HalfFloor:
    case capi::ICU4XFixedDecimalRoundingMode_HalfTrunc:
    case capi::ICU4XFixedDecimalRoundingMode_HalfEven:
      return static_cast<ICU4XFixedDecimalRoundingMode::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XFixedDecimalRoundingMode_HPP
