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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::DataError DataError::AsFFI() const {
  return static_cast<diplomat::capi::DataError>(value);
}

inline DataError DataError::FromFFI(diplomat::capi::DataError c_enum) {
  switch (c_enum) {
    case diplomat::capi::DataError_Unknown:
    case diplomat::capi::DataError_MarkerNotFound:
    case diplomat::capi::DataError_IdentifierNotFound:
    case diplomat::capi::DataError_InvalidRequest:
    case diplomat::capi::DataError_InconsistentData:
    case diplomat::capi::DataError_Downcast:
    case diplomat::capi::DataError_Deserialize:
    case diplomat::capi::DataError_Custom:
    case diplomat::capi::DataError_Io:
      return static_cast<DataError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // DataError_HPP
