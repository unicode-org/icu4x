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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::IsoTimeZoneFormat IsoTimeZoneFormat::AsFFI() const {
  return static_cast<diplomat::capi::IsoTimeZoneFormat>(value);
}

inline IsoTimeZoneFormat IsoTimeZoneFormat::FromFFI(diplomat::capi::IsoTimeZoneFormat c_enum) {
  switch (c_enum) {
    case diplomat::capi::IsoTimeZoneFormat_Basic:
    case diplomat::capi::IsoTimeZoneFormat_Extended:
    case diplomat::capi::IsoTimeZoneFormat_UtcBasic:
    case diplomat::capi::IsoTimeZoneFormat_UtcExtended:
      return static_cast<IsoTimeZoneFormat::Value>(c_enum);
    default:
      abort();
  }
}
#endif // IsoTimeZoneFormat_HPP
