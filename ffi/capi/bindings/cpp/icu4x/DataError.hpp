#ifndef icu4x_DataError_HPP
#define icu4x_DataError_HPP

#include "DataError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::DataError icu4x::DataError::AsFFI() const {
  return static_cast<icu4x::capi::DataError>(value);
}

inline icu4x::DataError icu4x::DataError::FromFFI(icu4x::capi::DataError c_enum) {
  switch (c_enum) {
    case icu4x::capi::DataError_Unknown:
    case icu4x::capi::DataError_MarkerNotFound:
    case icu4x::capi::DataError_IdentifierNotFound:
    case icu4x::capi::DataError_InvalidRequest:
    case icu4x::capi::DataError_InconsistentData:
    case icu4x::capi::DataError_Downcast:
    case icu4x::capi::DataError_Deserialize:
    case icu4x::capi::DataError_Custom:
    case icu4x::capi::DataError_Io:
      return static_cast<icu4x::DataError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_DataError_HPP
