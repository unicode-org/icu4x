#ifndef ICU4XFixedDecimalParseError_HPP
#define ICU4XFixedDecimalParseError_HPP

#include "ICU4XFixedDecimalParseError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalParseError.h"


inline capi::ICU4XFixedDecimalParseError ICU4XFixedDecimalParseError::AsFFI() const {
  return static_cast<capi::ICU4XFixedDecimalParseError>(value);
}

inline ICU4XFixedDecimalParseError ICU4XFixedDecimalParseError::FromFFI(capi::ICU4XFixedDecimalParseError c_enum) {
  switch (c_enum) {
    case capi::ICU4XFixedDecimalParseError_Unknown:
    case capi::ICU4XFixedDecimalParseError_Limit:
    case capi::ICU4XFixedDecimalParseError_Syntax:
      return static_cast<ICU4XFixedDecimalParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XFixedDecimalParseError_HPP
