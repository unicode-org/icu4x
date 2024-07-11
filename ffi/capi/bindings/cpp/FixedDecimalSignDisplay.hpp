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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::FixedDecimalSignDisplay FixedDecimalSignDisplay::AsFFI() const {
  return static_cast<capi::FixedDecimalSignDisplay>(value);
}

inline FixedDecimalSignDisplay FixedDecimalSignDisplay::FromFFI(capi::FixedDecimalSignDisplay c_enum) {
  switch (c_enum) {
    case capi::FixedDecimalSignDisplay_Auto:
    case capi::FixedDecimalSignDisplay_Never:
    case capi::FixedDecimalSignDisplay_Always:
    case capi::FixedDecimalSignDisplay_ExceptZero:
    case capi::FixedDecimalSignDisplay_Negative:
      return static_cast<FixedDecimalSignDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalSignDisplay_HPP
