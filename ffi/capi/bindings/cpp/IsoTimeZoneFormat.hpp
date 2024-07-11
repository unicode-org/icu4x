#ifndef IsoTimeZoneFormat_HPP
#define IsoTimeZoneFormat_HPP

#include "IsoTimeZoneFormat.d.hpp"

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


inline capi::IsoTimeZoneFormat IsoTimeZoneFormat::AsFFI() const {
  return static_cast<capi::IsoTimeZoneFormat>(value);
}

inline IsoTimeZoneFormat IsoTimeZoneFormat::FromFFI(capi::IsoTimeZoneFormat c_enum) {
  switch (c_enum) {
    case capi::IsoTimeZoneFormat_Basic:
    case capi::IsoTimeZoneFormat_Extended:
    case capi::IsoTimeZoneFormat_UtcBasic:
    case capi::IsoTimeZoneFormat_UtcExtended:
      return static_cast<IsoTimeZoneFormat::Value>(c_enum);
    default:
      abort();
  }
}
#endif // IsoTimeZoneFormat_HPP
