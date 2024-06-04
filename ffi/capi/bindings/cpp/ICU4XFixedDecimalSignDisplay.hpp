#ifndef ICU4XFixedDecimalSignDisplay_HPP
#define ICU4XFixedDecimalSignDisplay_HPP

#include "ICU4XFixedDecimalSignDisplay.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalSignDisplay.h"


inline capi::ICU4XFixedDecimalSignDisplay ICU4XFixedDecimalSignDisplay::AsFFI() const {
  return static_cast<capi::ICU4XFixedDecimalSignDisplay>(value);
}

inline ICU4XFixedDecimalSignDisplay ICU4XFixedDecimalSignDisplay::FromFFI(capi::ICU4XFixedDecimalSignDisplay c_enum) {
  switch (c_enum) {
    case capi::ICU4XFixedDecimalSignDisplay_Auto:
    case capi::ICU4XFixedDecimalSignDisplay_Never:
    case capi::ICU4XFixedDecimalSignDisplay_Always:
    case capi::ICU4XFixedDecimalSignDisplay_ExceptZero:
    case capi::ICU4XFixedDecimalSignDisplay_Negative:
      return static_cast<ICU4XFixedDecimalSignDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XFixedDecimalSignDisplay_HPP
