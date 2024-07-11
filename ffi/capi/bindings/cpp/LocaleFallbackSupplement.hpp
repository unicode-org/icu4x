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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::LocaleFallbackSupplement LocaleFallbackSupplement::AsFFI() const {
  return static_cast<capi::LocaleFallbackSupplement>(value);
}

inline LocaleFallbackSupplement LocaleFallbackSupplement::FromFFI(capi::LocaleFallbackSupplement c_enum) {
  switch (c_enum) {
    case capi::LocaleFallbackSupplement_None:
    case capi::LocaleFallbackSupplement_Collation:
      return static_cast<LocaleFallbackSupplement::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LocaleFallbackSupplement_HPP
