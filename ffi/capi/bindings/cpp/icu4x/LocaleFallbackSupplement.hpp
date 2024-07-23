#ifndef icu4x_LocaleFallbackSupplement_HPP
#define icu4x_LocaleFallbackSupplement_HPP

#include "LocaleFallbackSupplement.d.hpp"

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

inline icu4x::capi::LocaleFallbackSupplement icu4x::LocaleFallbackSupplement::AsFFI() const {
  return static_cast<icu4x::capi::LocaleFallbackSupplement>(value);
}

inline icu4x::LocaleFallbackSupplement icu4x::LocaleFallbackSupplement::FromFFI(icu4x::capi::LocaleFallbackSupplement c_enum) {
  switch (c_enum) {
    case icu4x::capi::LocaleFallbackSupplement_None:
    case icu4x::capi::LocaleFallbackSupplement_Collation:
      return static_cast<icu4x::LocaleFallbackSupplement::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_LocaleFallbackSupplement_HPP
