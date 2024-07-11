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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::IsoTimeZoneMinuteDisplay IsoTimeZoneMinuteDisplay::AsFFI() const {
  return static_cast<capi::IsoTimeZoneMinuteDisplay>(value);
}

inline IsoTimeZoneMinuteDisplay IsoTimeZoneMinuteDisplay::FromFFI(capi::IsoTimeZoneMinuteDisplay c_enum) {
  switch (c_enum) {
    case capi::IsoTimeZoneMinuteDisplay_Required:
    case capi::IsoTimeZoneMinuteDisplay_Optional:
      return static_cast<IsoTimeZoneMinuteDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // IsoTimeZoneMinuteDisplay_HPP
