#ifndef ICU4XIsoTimeZoneOptions_HPP
#define ICU4XIsoTimeZoneOptions_HPP

#include "ICU4XIsoTimeZoneOptions.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XIsoTimeZoneFormat.hpp"
#include "ICU4XIsoTimeZoneMinuteDisplay.hpp"
#include "ICU4XIsoTimeZoneSecondDisplay.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::ICU4XIsoTimeZoneOptions ICU4XIsoTimeZoneOptions::AsFFI() const {
  return capi::ICU4XIsoTimeZoneOptions {
    .format = format.AsFFI(),
    .minutes = minutes.AsFFI(),
    .seconds = seconds.AsFFI(),
  };
}

inline ICU4XIsoTimeZoneOptions ICU4XIsoTimeZoneOptions::FromFFI(capi::ICU4XIsoTimeZoneOptions c_struct) {
  return ICU4XIsoTimeZoneOptions {
    .format = ICU4XIsoTimeZoneFormat::FromFFI(c_struct.format),
    .minutes = ICU4XIsoTimeZoneMinuteDisplay::FromFFI(c_struct.minutes),
    .seconds = ICU4XIsoTimeZoneSecondDisplay::FromFFI(c_struct.seconds),
  };
}


#endif // ICU4XIsoTimeZoneOptions_HPP
