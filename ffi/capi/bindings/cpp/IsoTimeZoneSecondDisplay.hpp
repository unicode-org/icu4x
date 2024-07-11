#ifndef IsoTimeZoneSecondDisplay_HPP
#define IsoTimeZoneSecondDisplay_HPP

#include "IsoTimeZoneSecondDisplay.d.hpp"

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


inline capi::IsoTimeZoneSecondDisplay IsoTimeZoneSecondDisplay::AsFFI() const {
  return static_cast<capi::IsoTimeZoneSecondDisplay>(value);
}

inline IsoTimeZoneSecondDisplay IsoTimeZoneSecondDisplay::FromFFI(capi::IsoTimeZoneSecondDisplay c_enum) {
  switch (c_enum) {
    case capi::IsoTimeZoneSecondDisplay_Optional:
    case capi::IsoTimeZoneSecondDisplay_Never:
      return static_cast<IsoTimeZoneSecondDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // IsoTimeZoneSecondDisplay_HPP
