#ifndef LocaleDirection_HPP
#define LocaleDirection_HPP

#include "LocaleDirection.d.hpp"

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


inline capi::LocaleDirection LocaleDirection::AsFFI() const {
  return static_cast<capi::LocaleDirection>(value);
}

inline LocaleDirection LocaleDirection::FromFFI(capi::LocaleDirection c_enum) {
  switch (c_enum) {
    case capi::LocaleDirection_LeftToRight:
    case capi::LocaleDirection_RightToLeft:
    case capi::LocaleDirection_Unknown:
      return static_cast<LocaleDirection::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LocaleDirection_HPP
