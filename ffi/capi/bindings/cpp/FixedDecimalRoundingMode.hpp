#ifndef FixedDecimalRoundingMode_HPP
#define FixedDecimalRoundingMode_HPP

#include "FixedDecimalRoundingMode.d.hpp"

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


inline capi::FixedDecimalRoundingMode FixedDecimalRoundingMode::AsFFI() const {
  return static_cast<capi::FixedDecimalRoundingMode>(value);
}

inline FixedDecimalRoundingMode FixedDecimalRoundingMode::FromFFI(capi::FixedDecimalRoundingMode c_enum) {
  switch (c_enum) {
    case capi::FixedDecimalRoundingMode_Ceil:
    case capi::FixedDecimalRoundingMode_Expand:
    case capi::FixedDecimalRoundingMode_Floor:
    case capi::FixedDecimalRoundingMode_Trunc:
    case capi::FixedDecimalRoundingMode_HalfCeil:
    case capi::FixedDecimalRoundingMode_HalfExpand:
    case capi::FixedDecimalRoundingMode_HalfFloor:
    case capi::FixedDecimalRoundingMode_HalfTrunc:
    case capi::FixedDecimalRoundingMode_HalfEven:
      return static_cast<FixedDecimalRoundingMode::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalRoundingMode_HPP
