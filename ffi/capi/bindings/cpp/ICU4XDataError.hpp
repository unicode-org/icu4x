#ifndef ICU4XDataError_HPP
#define ICU4XDataError_HPP

#include "ICU4XDataError.d.hpp"

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


inline capi::ICU4XDataError ICU4XDataError::AsFFI() const {
  return static_cast<capi::ICU4XDataError>(value);
}

inline ICU4XDataError ICU4XDataError::FromFFI(capi::ICU4XDataError c_enum) {
  switch (c_enum) {
    case capi::ICU4XDataError_Unknown:
    case capi::ICU4XDataError_MarkerNotFound:
    case capi::ICU4XDataError_IdentifierNotFound:
    case capi::ICU4XDataError_InvalidRequest:
    case capi::ICU4XDataError_InconsistentData:
    case capi::ICU4XDataError_Downcast:
    case capi::ICU4XDataError_Deserialize:
    case capi::ICU4XDataError_Custom:
    case capi::ICU4XDataError_Io:
      return static_cast<ICU4XDataError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XDataError_HPP
