#ifndef icu4x_TimeLength_HPP
#define icu4x_TimeLength_HPP

#include "TimeLength.d.hpp"

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

inline icu4x::capi::TimeLength icu4x::TimeLength::AsFFI() const {
  return static_cast<icu4x::capi::TimeLength>(value);
}

inline icu4x::TimeLength icu4x::TimeLength::FromFFI(icu4x::capi::TimeLength c_enum) {
  switch (c_enum) {
    case icu4x::capi::TimeLength_Full:
    case icu4x::capi::TimeLength_Long:
    case icu4x::capi::TimeLength_Medium:
    case icu4x::capi::TimeLength_Short:
      return static_cast<icu4x::TimeLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_TimeLength_HPP
