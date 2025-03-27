#ifndef icu4x_ZoneStyle_HPP
#define icu4x_ZoneStyle_HPP

#include "ZoneStyle.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::ZoneStyle icu4x::ZoneStyle::AsFFI() const {
  return static_cast<icu4x::capi::ZoneStyle>(value);
}

inline icu4x::ZoneStyle icu4x::ZoneStyle::FromFFI(icu4x::capi::ZoneStyle c_enum) {
  switch (c_enum) {
    case icu4x::capi::ZoneStyle_SpecificLong:
    case icu4x::capi::ZoneStyle_SpecificShort:
    case icu4x::capi::ZoneStyle_LocalizedOffsetLong:
    case icu4x::capi::ZoneStyle_LocalizedOffsetShort:
    case icu4x::capi::ZoneStyle_GenericLong:
    case icu4x::capi::ZoneStyle_GenericShort:
    case icu4x::capi::ZoneStyle_Location:
    case icu4x::capi::ZoneStyle_ExemplarCity:
      return static_cast<icu4x::ZoneStyle::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_ZoneStyle_HPP
