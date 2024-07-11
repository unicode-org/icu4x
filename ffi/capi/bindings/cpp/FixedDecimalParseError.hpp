#ifndef FixedDecimalParseError_HPP
#define FixedDecimalParseError_HPP

#include "FixedDecimalParseError.d.hpp"

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


inline capi::FixedDecimalParseError FixedDecimalParseError::AsFFI() const {
  return static_cast<capi::FixedDecimalParseError>(value);
}

inline FixedDecimalParseError FixedDecimalParseError::FromFFI(capi::FixedDecimalParseError c_enum) {
  switch (c_enum) {
    case capi::FixedDecimalParseError_Unknown:
    case capi::FixedDecimalParseError_Limit:
    case capi::FixedDecimalParseError_Syntax:
      return static_cast<FixedDecimalParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalParseError_HPP
