#ifndef DataError_HPP
#define DataError_HPP

#include "DataError.d.hpp"

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


inline capi::DataError DataError::AsFFI() const {
  return static_cast<capi::DataError>(value);
}

inline DataError DataError::FromFFI(capi::DataError c_enum) {
  switch (c_enum) {
    case capi::DataError_Unknown:
    case capi::DataError_MarkerNotFound:
    case capi::DataError_IdentifierNotFound:
    case capi::DataError_InvalidRequest:
    case capi::DataError_InconsistentData:
    case capi::DataError_Downcast:
    case capi::DataError_Deserialize:
    case capi::DataError_Custom:
    case capi::DataError_Io:
      return static_cast<DataError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // DataError_HPP
