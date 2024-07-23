#ifndef icu4x_IsoTimeZoneFormat_HPP
#define icu4x_IsoTimeZoneFormat_HPP

#include "IsoTimeZoneFormat.d.hpp"

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

inline icu4x::capi::IsoTimeZoneFormat icu4x::IsoTimeZoneFormat::AsFFI() const {
  return static_cast<icu4x::capi::IsoTimeZoneFormat>(value);
}

inline icu4x::IsoTimeZoneFormat icu4x::IsoTimeZoneFormat::FromFFI(icu4x::capi::IsoTimeZoneFormat c_enum) {
  switch (c_enum) {
    case icu4x::capi::IsoTimeZoneFormat_Basic:
    case icu4x::capi::IsoTimeZoneFormat_Extended:
    case icu4x::capi::IsoTimeZoneFormat_UtcBasic:
    case icu4x::capi::IsoTimeZoneFormat_UtcExtended:
      return static_cast<icu4x::IsoTimeZoneFormat::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_IsoTimeZoneFormat_HPP
