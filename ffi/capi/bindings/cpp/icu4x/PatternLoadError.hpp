#ifndef icu4x_PatternLoadError_HPP
#define icu4x_PatternLoadError_HPP

#include "PatternLoadError.d.hpp"

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

inline icu4x::capi::PatternLoadError icu4x::PatternLoadError::AsFFI() const {
  return static_cast<icu4x::capi::PatternLoadError>(value);
}

inline icu4x::PatternLoadError icu4x::PatternLoadError::FromFFI(icu4x::capi::PatternLoadError c_enum) {
  switch (c_enum) {
    case icu4x::capi::PatternLoadError_Unknown:
    case icu4x::capi::PatternLoadError_UnsupportedLength:
    case icu4x::capi::PatternLoadError_DuplicateField:
    case icu4x::capi::PatternLoadError_TypeTooSpecific:
    case icu4x::capi::PatternLoadError_DataMarkerNotFound:
    case icu4x::capi::PatternLoadError_DataIdentifierNotFound:
    case icu4x::capi::PatternLoadError_DataInvalidRequest:
    case icu4x::capi::PatternLoadError_DataInconsistentData:
    case icu4x::capi::PatternLoadError_DataDowncast:
    case icu4x::capi::PatternLoadError_DataDeserialize:
    case icu4x::capi::PatternLoadError_DataCustom:
    case icu4x::capi::PatternLoadError_DataIo:
      return static_cast<icu4x::PatternLoadError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_PatternLoadError_HPP
