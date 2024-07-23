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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::LocaleDirection LocaleDirection::AsFFI() const {
  return static_cast<diplomat::capi::LocaleDirection>(value);
}

inline LocaleDirection LocaleDirection::FromFFI(diplomat::capi::LocaleDirection c_enum) {
  switch (c_enum) {
    case diplomat::capi::LocaleDirection_LeftToRight:
    case diplomat::capi::LocaleDirection_RightToLeft:
    case diplomat::capi::LocaleDirection_Unknown:
      return static_cast<LocaleDirection::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LocaleDirection_HPP
