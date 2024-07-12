#ifndef FixedDecimalSignDisplay_HPP
#define FixedDecimalSignDisplay_HPP

#include "FixedDecimalSignDisplay.d.hpp"

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

inline diplomat::capi::FixedDecimalSignDisplay FixedDecimalSignDisplay::AsFFI() const {
  return static_cast<diplomat::capi::FixedDecimalSignDisplay>(value);
}

inline FixedDecimalSignDisplay FixedDecimalSignDisplay::FromFFI(diplomat::capi::FixedDecimalSignDisplay c_enum) {
  switch (c_enum) {
    case diplomat::capi::FixedDecimalSignDisplay_Auto:
    case diplomat::capi::FixedDecimalSignDisplay_Never:
    case diplomat::capi::FixedDecimalSignDisplay_Always:
    case diplomat::capi::FixedDecimalSignDisplay_ExceptZero:
    case diplomat::capi::FixedDecimalSignDisplay_Negative:
      return static_cast<FixedDecimalSignDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalSignDisplay_HPP
