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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::FixedDecimalLimitError FixedDecimalLimitError::AsFFI() const {
  return static_cast<capi::FixedDecimalLimitError>(value);
}

inline FixedDecimalLimitError FixedDecimalLimitError::FromFFI(capi::FixedDecimalLimitError c_enum) {
  switch (c_enum) {
    case capi::FixedDecimalLimitError_TodoZst:
      return static_cast<FixedDecimalLimitError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FixedDecimalLimitError_HPP
