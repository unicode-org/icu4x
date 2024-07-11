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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::LocaleParseError LocaleParseError::AsFFI() const {
  return static_cast<capi::LocaleParseError>(value);
}

inline LocaleParseError LocaleParseError::FromFFI(capi::LocaleParseError c_enum) {
  switch (c_enum) {
    case capi::LocaleParseError_Unknown:
    case capi::LocaleParseError_Language:
    case capi::LocaleParseError_Subtag:
    case capi::LocaleParseError_Extension:
      return static_cast<LocaleParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LocaleParseError_HPP
