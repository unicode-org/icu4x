#ifndef ICU4XLocaleParseError_HPP
#define ICU4XLocaleParseError_HPP

#include "ICU4XLocaleParseError.d.hpp"

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


inline capi::ICU4XLocaleParseError ICU4XLocaleParseError::AsFFI() const {
  return static_cast<capi::ICU4XLocaleParseError>(value);
}

inline ICU4XLocaleParseError ICU4XLocaleParseError::FromFFI(capi::ICU4XLocaleParseError c_enum) {
  switch (c_enum) {
    case capi::ICU4XLocaleParseError_Unknown:
    case capi::ICU4XLocaleParseError_Language:
    case capi::ICU4XLocaleParseError_Subtag:
    case capi::ICU4XLocaleParseError_Extension:
      return static_cast<ICU4XLocaleParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XLocaleParseError_HPP
