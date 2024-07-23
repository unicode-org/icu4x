#ifndef LocaleParseError_HPP
#define LocaleParseError_HPP

#include "LocaleParseError.d.hpp"

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

inline diplomat::capi::LocaleParseError LocaleParseError::AsFFI() const {
  return static_cast<diplomat::capi::LocaleParseError>(value);
}

inline LocaleParseError LocaleParseError::FromFFI(diplomat::capi::LocaleParseError c_enum) {
  switch (c_enum) {
    case diplomat::capi::LocaleParseError_Unknown:
    case diplomat::capi::LocaleParseError_Language:
    case diplomat::capi::LocaleParseError_Subtag:
    case diplomat::capi::LocaleParseError_Extension:
      return static_cast<LocaleParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LocaleParseError_HPP
