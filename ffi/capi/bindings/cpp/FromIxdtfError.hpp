#ifndef FromIxdtfError_HPP
#define FromIxdtfError_HPP

#include "FromIxdtfError.d.hpp"

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

inline diplomat::capi::FromIxdtfError FromIxdtfError::AsFFI() const {
  return static_cast<diplomat::capi::FromIxdtfError>(value);
}

inline FromIxdtfError FromIxdtfError::FromFFI(diplomat::capi::FromIxdtfError c_enum) {
  switch (c_enum) {
    case diplomat::capi::FromIxdtfError_Unknown:
    case diplomat::capi::FromIxdtfError_InvalidSyntax:
    case diplomat::capi::FromIxdtfError_OutOfRange:
    case diplomat::capi::FromIxdtfError_MissingFields:
    case diplomat::capi::FromIxdtfError_UnknownCalendar:
      return static_cast<FromIxdtfError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // FromIxdtfError_HPP
