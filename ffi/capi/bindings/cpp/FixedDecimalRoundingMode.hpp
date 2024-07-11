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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::FixedDecimalRoundingMode FixedDecimalRoundingMode::AsFFI() const {
  return static_cast<diplomat::capi::FixedDecimalRoundingMode>(value);
}

inline FixedDecimalRoundingMode FixedDecimalRoundingMode::FromFFI(diplomat::capi::FixedDecimalRoundingMode c_enum) {
  switch (c_enum) {
    case diplomat::capi::FixedDecimalRoundingMode_Ceil:
    case diplomat::capi::FixedDecimalRoundingMode_Expand:
    case diplomat::capi::FixedDecimalRoundingMode_Floor:
    case diplomat::capi::FixedDecimalRoundingMode_Trunc:
    case diplomat::capi::FixedDecimalRoundingMode_HalfCeil:
    case diplomat::capi::FixedDecimalRoundingMode_HalfExpand:
    case diplomat::capi::FixedDecimalRoundingMode_HalfFloor:
    case diplomat::capi::FixedDecimalRoundingMode_HalfTrunc:
    case diplomat::capi::FixedDecimalRoundingMode_HalfEven:
      return static_cast<FixedDecimalRoundingMode::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalRoundingMode_HPP
