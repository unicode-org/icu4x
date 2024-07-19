#ifndef icu4x_IsoTimeZoneSecondDisplay_HPP
#define icu4x_IsoTimeZoneSecondDisplay_HPP

#include "IsoTimeZoneSecondDisplay.d.hpp"

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

inline icu4x::capi::IsoTimeZoneSecondDisplay icu4x::IsoTimeZoneSecondDisplay::AsFFI() const {
  return static_cast<icu4x::capi::IsoTimeZoneSecondDisplay>(value);
}

inline icu4x::IsoTimeZoneSecondDisplay icu4x::IsoTimeZoneSecondDisplay::FromFFI(icu4x::capi::IsoTimeZoneSecondDisplay c_enum) {
  switch (c_enum) {
    case icu4x::capi::IsoTimeZoneSecondDisplay_Optional:
    case icu4x::capi::IsoTimeZoneSecondDisplay_Never:
      return static_cast<icu4x::IsoTimeZoneSecondDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_IsoTimeZoneSecondDisplay_HPP
