#ifndef IsoTimeZoneOptions_HPP
#define IsoTimeZoneOptions_HPP

#include "IsoTimeZoneOptions.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "IsoTimeZoneFormat.hpp"
#include "IsoTimeZoneMinuteDisplay.hpp"
#include "IsoTimeZoneSecondDisplay.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::IsoTimeZoneOptions IsoTimeZoneOptions::AsFFI() const {
  return diplomat::capi::IsoTimeZoneOptions {
    .format = format.AsFFI(),
    .minutes = minutes.AsFFI(),
    .seconds = seconds.AsFFI(),
  };
}

inline IsoTimeZoneOptions IsoTimeZoneOptions::FromFFI(diplomat::capi::IsoTimeZoneOptions c_struct) {
  return IsoTimeZoneOptions {
    .format = IsoTimeZoneFormat::FromFFI(c_struct.format),
    .minutes = IsoTimeZoneMinuteDisplay::FromFFI(c_struct.minutes),
    .seconds = IsoTimeZoneSecondDisplay::FromFFI(c_struct.seconds),
  };
}


#endif // IsoTimeZoneOptions_HPP
