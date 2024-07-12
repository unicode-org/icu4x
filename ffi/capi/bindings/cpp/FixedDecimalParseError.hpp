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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::FixedDecimalParseError FixedDecimalParseError::AsFFI() const {
  return static_cast<diplomat::capi::FixedDecimalParseError>(value);
}

inline FixedDecimalParseError FixedDecimalParseError::FromFFI(diplomat::capi::FixedDecimalParseError c_enum) {
  switch (c_enum) {
    case diplomat::capi::FixedDecimalParseError_Unknown:
    case diplomat::capi::FixedDecimalParseError_Limit:
    case diplomat::capi::FixedDecimalParseError_Syntax:
      return static_cast<FixedDecimalParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalParseError_HPP
