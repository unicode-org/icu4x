#ifndef icu4x_LocaleDirection_HPP
#define icu4x_LocaleDirection_HPP

#include "LocaleDirection.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::LocaleDirection icu4x::LocaleDirection::AsFFI() const {
  return static_cast<icu4x::capi::LocaleDirection>(value);
}

inline icu4x::LocaleDirection icu4x::LocaleDirection::FromFFI(icu4x::capi::LocaleDirection c_enum) {
  switch (c_enum) {
    case icu4x::capi::LocaleDirection_LeftToRight:
    case icu4x::capi::LocaleDirection_RightToLeft:
    case icu4x::capi::LocaleDirection_Unknown:
      return static_cast<icu4x::LocaleDirection::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_LocaleDirection_HPP
