#ifndef icu4x_DateFields_HPP
#define icu4x_DateFields_HPP

#include "DateFields.d.hpp"

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

inline icu4x::capi::DateFields icu4x::DateFields::AsFFI() const {
  return static_cast<icu4x::capi::DateFields>(value);
}

inline icu4x::DateFields icu4x::DateFields::FromFFI(icu4x::capi::DateFields c_enum) {
  switch (c_enum) {
    case icu4x::capi::DateFields_D:
    case icu4x::capi::DateFields_MD:
    case icu4x::capi::DateFields_YMD:
    case icu4x::capi::DateFields_DE:
    case icu4x::capi::DateFields_MDE:
    case icu4x::capi::DateFields_YMDE:
    case icu4x::capi::DateFields_E:
    case icu4x::capi::DateFields_M:
    case icu4x::capi::DateFields_YM:
    case icu4x::capi::DateFields_Y:
      return static_cast<icu4x::DateFields::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_DateFields_HPP
