#ifndef LocaleFallbackSupplement_HPP
#define LocaleFallbackSupplement_HPP

#include "LocaleFallbackSupplement.d.hpp"

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

inline diplomat::capi::LocaleFallbackSupplement LocaleFallbackSupplement::AsFFI() const {
  return static_cast<diplomat::capi::LocaleFallbackSupplement>(value);
}

inline LocaleFallbackSupplement LocaleFallbackSupplement::FromFFI(diplomat::capi::LocaleFallbackSupplement c_enum) {
  switch (c_enum) {
    case diplomat::capi::LocaleFallbackSupplement_None:
    case diplomat::capi::LocaleFallbackSupplement_Collation:
      return static_cast<LocaleFallbackSupplement::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LocaleFallbackSupplement_HPP
