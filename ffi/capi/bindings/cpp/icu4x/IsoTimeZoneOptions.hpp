#ifndef icu4x_IsoTimeZoneOptions_HPP
#define icu4x_IsoTimeZoneOptions_HPP

#include "IsoTimeZoneOptions.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "IsoTimeZoneFormat.hpp"
#include "IsoTimeZoneMinuteDisplay.hpp"
#include "IsoTimeZoneSecondDisplay.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::IsoTimeZoneOptions icu4x::IsoTimeZoneOptions::AsFFI() const {
  return icu4x::capi::IsoTimeZoneOptions {
    /* .format = */ format.AsFFI(),
    /* .minutes = */ minutes.AsFFI(),
    /* .seconds = */ seconds.AsFFI(),
  };
}

inline icu4x::IsoTimeZoneOptions icu4x::IsoTimeZoneOptions::FromFFI(icu4x::capi::IsoTimeZoneOptions c_struct) {
  return icu4x::IsoTimeZoneOptions {
    /* .format = */ icu4x::IsoTimeZoneFormat::FromFFI(c_struct.format),
    /* .minutes = */ icu4x::IsoTimeZoneMinuteDisplay::FromFFI(c_struct.minutes),
    /* .seconds = */ icu4x::IsoTimeZoneSecondDisplay::FromFFI(c_struct.seconds),
  };
}


#endif // icu4x_IsoTimeZoneOptions_HPP
