#ifndef ICU4XIsoTimeZoneSecondDisplay_HPP
#define ICU4XIsoTimeZoneSecondDisplay_HPP

#include "ICU4XIsoTimeZoneSecondDisplay.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XIsoTimeZoneSecondDisplay.h"


inline capi::ICU4XIsoTimeZoneSecondDisplay ICU4XIsoTimeZoneSecondDisplay::AsFFI() const {
  return static_cast<capi::ICU4XIsoTimeZoneSecondDisplay>(value);
}

inline ICU4XIsoTimeZoneSecondDisplay ICU4XIsoTimeZoneSecondDisplay::FromFFI(capi::ICU4XIsoTimeZoneSecondDisplay c_enum) {
  switch (c_enum) {
    case capi::ICU4XIsoTimeZoneSecondDisplay_Optional:
    case capi::ICU4XIsoTimeZoneSecondDisplay_Never:
      return static_cast<ICU4XIsoTimeZoneSecondDisplay::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XIsoTimeZoneSecondDisplay_HPP
