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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::IsoTimeZoneSecondDisplay IsoTimeZoneSecondDisplay::AsFFI() const {
  return static_cast<diplomat::capi::IsoTimeZoneSecondDisplay>(value);
}

inline IsoTimeZoneSecondDisplay IsoTimeZoneSecondDisplay::FromFFI(diplomat::capi::IsoTimeZoneSecondDisplay c_enum) {
  switch (c_enum) {
    case diplomat::capi::IsoTimeZoneSecondDisplay_Optional:
    case diplomat::capi::IsoTimeZoneSecondDisplay_Never:
      return static_cast<IsoTimeZoneSecondDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // IsoTimeZoneSecondDisplay_HPP
