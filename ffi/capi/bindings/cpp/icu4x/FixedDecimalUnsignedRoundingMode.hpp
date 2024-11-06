#ifndef icu4x_FixedDecimalUnsignedRoundingMode_HPP
#define icu4x_FixedDecimalUnsignedRoundingMode_HPP

#include "FixedDecimalUnsignedRoundingMode.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::FixedDecimalUnsignedRoundingMode icu4x::FixedDecimalUnsignedRoundingMode::AsFFI() const {
  return static_cast<icu4x::capi::FixedDecimalUnsignedRoundingMode>(value);
}

inline icu4x::FixedDecimalUnsignedRoundingMode icu4x::FixedDecimalUnsignedRoundingMode::FromFFI(icu4x::capi::FixedDecimalUnsignedRoundingMode c_enum) {
  switch (c_enum) {
    case icu4x::capi::FixedDecimalUnsignedRoundingMode_Expand:
    case icu4x::capi::FixedDecimalUnsignedRoundingMode_Trunc:
    case icu4x::capi::FixedDecimalUnsignedRoundingMode_HalfExpand:
    case icu4x::capi::FixedDecimalUnsignedRoundingMode_HalfTrunc:
    case icu4x::capi::FixedDecimalUnsignedRoundingMode_HalfEven:
      return static_cast<icu4x::FixedDecimalUnsignedRoundingMode::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_FixedDecimalUnsignedRoundingMode_HPP
