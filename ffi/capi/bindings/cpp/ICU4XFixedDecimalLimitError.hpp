#ifndef ICU4XFixedDecimalLimitError_HPP
#define ICU4XFixedDecimalLimitError_HPP

#include "ICU4XFixedDecimalLimitError.d.hpp"

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


inline capi::ICU4XFixedDecimalLimitError ICU4XFixedDecimalLimitError::AsFFI() const {
  return static_cast<capi::ICU4XFixedDecimalLimitError>(value);
}

inline ICU4XFixedDecimalLimitError ICU4XFixedDecimalLimitError::FromFFI(capi::ICU4XFixedDecimalLimitError c_enum) {
  switch (c_enum) {
    case capi::ICU4XFixedDecimalLimitError_TodoZst:
      return static_cast<ICU4XFixedDecimalLimitError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XFixedDecimalLimitError_HPP
