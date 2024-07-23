#ifndef IsoTimeZoneMinuteDisplay_HPP
#define IsoTimeZoneMinuteDisplay_HPP

#include "IsoTimeZoneMinuteDisplay.d.hpp"

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

inline diplomat::capi::IsoTimeZoneMinuteDisplay IsoTimeZoneMinuteDisplay::AsFFI() const {
  return static_cast<diplomat::capi::IsoTimeZoneMinuteDisplay>(value);
}

inline IsoTimeZoneMinuteDisplay IsoTimeZoneMinuteDisplay::FromFFI(diplomat::capi::IsoTimeZoneMinuteDisplay c_enum) {
  switch (c_enum) {
    case diplomat::capi::IsoTimeZoneMinuteDisplay_Required:
    case diplomat::capi::IsoTimeZoneMinuteDisplay_Optional:
      return static_cast<IsoTimeZoneMinuteDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // IsoTimeZoneMinuteDisplay_HPP
