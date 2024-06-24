#ifndef ICU4XIsoTimeZoneMinuteDisplay_HPP
#define ICU4XIsoTimeZoneMinuteDisplay_HPP

#include "ICU4XIsoTimeZoneMinuteDisplay.d.hpp"

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


inline capi::ICU4XIsoTimeZoneMinuteDisplay ICU4XIsoTimeZoneMinuteDisplay::AsFFI() const {
  return static_cast<capi::ICU4XIsoTimeZoneMinuteDisplay>(value);
}

inline ICU4XIsoTimeZoneMinuteDisplay ICU4XIsoTimeZoneMinuteDisplay::FromFFI(capi::ICU4XIsoTimeZoneMinuteDisplay c_enum) {
  switch (c_enum) {
    case capi::ICU4XIsoTimeZoneMinuteDisplay_Required:
    case capi::ICU4XIsoTimeZoneMinuteDisplay_Optional:
      return static_cast<ICU4XIsoTimeZoneMinuteDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XIsoTimeZoneMinuteDisplay_HPP
