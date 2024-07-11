#ifndef FixedDecimalLimitError_HPP
#define FixedDecimalLimitError_HPP

#include "FixedDecimalLimitError.d.hpp"

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

inline diplomat::capi::FixedDecimalLimitError FixedDecimalLimitError::AsFFI() const {
  return static_cast<diplomat::capi::FixedDecimalLimitError>(value);
}

inline FixedDecimalLimitError FixedDecimalLimitError::FromFFI(diplomat::capi::FixedDecimalLimitError c_enum) {
  switch (c_enum) {
    case diplomat::capi::FixedDecimalLimitError_TodoZst:
      return static_cast<FixedDecimalLimitError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalLimitError_HPP
