#ifndef LocaleFallbackPriority_HPP
#define LocaleFallbackPriority_HPP

#include "LocaleFallbackPriority.d.hpp"

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

inline diplomat::capi::LocaleFallbackPriority LocaleFallbackPriority::AsFFI() const {
  return static_cast<diplomat::capi::LocaleFallbackPriority>(value);
}

inline LocaleFallbackPriority LocaleFallbackPriority::FromFFI(diplomat::capi::LocaleFallbackPriority c_enum) {
  switch (c_enum) {
    case diplomat::capi::LocaleFallbackPriority_Language:
    case diplomat::capi::LocaleFallbackPriority_Region:
    case diplomat::capi::LocaleFallbackPriority_Collation:
      return static_cast<LocaleFallbackPriority::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LocaleFallbackPriority_HPP
