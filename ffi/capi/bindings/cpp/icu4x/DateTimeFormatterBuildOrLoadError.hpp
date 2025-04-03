#ifndef icu4x_DateTimeFormatterBuildOrLoadError_HPP
#define icu4x_DateTimeFormatterBuildOrLoadError_HPP

#include "DateTimeFormatterBuildOrLoadError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::DateTimeFormatterBuildOrLoadError icu4x::DateTimeFormatterBuildOrLoadError::AsFFI() const {
  return static_cast<icu4x::capi::DateTimeFormatterBuildOrLoadError>(value);
}

inline icu4x::DateTimeFormatterBuildOrLoadError icu4x::DateTimeFormatterBuildOrLoadError::FromFFI(icu4x::capi::DateTimeFormatterBuildOrLoadError c_enum) {
  switch (c_enum) {
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_Unknown:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_DataMarkerNotFound:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_DataIdentifierNotFound:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_DataInvalidRequest:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_DataInconsistentData:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_DataDowncast:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_DataDeserialize:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_DataCustom:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_DataIo:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_MissingDateFields:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_MissingTimePrecision:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_MissingZoneStyle:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_InvalidDateFields:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_SuperfluousOptions:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_UnsupportedLength:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_ConflictingField:
    case icu4x::capi::DateTimeFormatterBuildOrLoadError_FormatterTooSpecific:
      return static_cast<icu4x::DateTimeFormatterBuildOrLoadError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_DateTimeFormatterBuildOrLoadError_HPP
