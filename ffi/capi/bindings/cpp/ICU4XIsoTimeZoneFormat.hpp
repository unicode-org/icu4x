#ifndef ICU4XIsoTimeZoneFormat_HPP
#define ICU4XIsoTimeZoneFormat_HPP

#include "ICU4XIsoTimeZoneFormat.d.hpp"

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


inline capi::ICU4XIsoTimeZoneFormat ICU4XIsoTimeZoneFormat::AsFFI() const {
  return static_cast<capi::ICU4XIsoTimeZoneFormat>(value);
}

inline ICU4XIsoTimeZoneFormat ICU4XIsoTimeZoneFormat::FromFFI(capi::ICU4XIsoTimeZoneFormat c_enum) {
  switch (c_enum) {
    case capi::ICU4XIsoTimeZoneFormat_Basic:
    case capi::ICU4XIsoTimeZoneFormat_Extended:
    case capi::ICU4XIsoTimeZoneFormat_UtcBasic:
    case capi::ICU4XIsoTimeZoneFormat_UtcExtended:
      return static_cast<ICU4XIsoTimeZoneFormat::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XIsoTimeZoneFormat_HPP
