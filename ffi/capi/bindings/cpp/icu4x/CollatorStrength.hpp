#ifndef icu4x_CollatorStrength_HPP
#define icu4x_CollatorStrength_HPP

#include "CollatorStrength.d.hpp"

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

inline icu4x::capi::CollatorStrength icu4x::CollatorStrength::AsFFI() const {
  return static_cast<icu4x::capi::CollatorStrength>(value);
}

inline icu4x::CollatorStrength icu4x::CollatorStrength::FromFFI(icu4x::capi::CollatorStrength c_enum) {
  switch (c_enum) {
    case icu4x::capi::CollatorStrength_Auto:
    case icu4x::capi::CollatorStrength_Primary:
    case icu4x::capi::CollatorStrength_Secondary:
    case icu4x::capi::CollatorStrength_Tertiary:
    case icu4x::capi::CollatorStrength_Quaternary:
    case icu4x::capi::CollatorStrength_Identical:
      return static_cast<icu4x::CollatorStrength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_CollatorStrength_HPP
