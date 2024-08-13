#ifndef icu4x_IsoTimeZoneMinuteDisplay_HPP
#define icu4x_IsoTimeZoneMinuteDisplay_HPP

#include "IsoTimeZoneMinuteDisplay.d.hpp"

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

inline icu4x::capi::IsoTimeZoneMinuteDisplay icu4x::IsoTimeZoneMinuteDisplay::AsFFI() const {
  return static_cast<icu4x::capi::IsoTimeZoneMinuteDisplay>(value);
}

inline icu4x::IsoTimeZoneMinuteDisplay icu4x::IsoTimeZoneMinuteDisplay::FromFFI(icu4x::capi::IsoTimeZoneMinuteDisplay c_enum) {
  switch (c_enum) {
    case icu4x::capi::IsoTimeZoneMinuteDisplay_Required:
    case icu4x::capi::IsoTimeZoneMinuteDisplay_Optional:
      return static_cast<icu4x::IsoTimeZoneMinuteDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_IsoTimeZoneMinuteDisplay_HPP
