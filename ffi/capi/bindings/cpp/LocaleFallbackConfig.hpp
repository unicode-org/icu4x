#ifndef LocaleFallbackConfig_HPP
#define LocaleFallbackConfig_HPP

#include "LocaleFallbackConfig.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "LocaleFallbackPriority.hpp"
#include "LocaleFallbackSupplement.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::LocaleFallbackConfig LocaleFallbackConfig::AsFFI() const {
  return diplomat::capi::LocaleFallbackConfig {
    .priority = priority.AsFFI(),
    .extension_key = { .data = extension_key.data(), .len = extension_key.size() },
    .fallback_supplement = fallback_supplement.AsFFI(),
  };
}

inline LocaleFallbackConfig LocaleFallbackConfig::FromFFI(diplomat::capi::LocaleFallbackConfig c_struct) {
  return LocaleFallbackConfig {
    .priority = LocaleFallbackPriority::FromFFI(c_struct.priority),
    .extension_key = std::string_view(c_struct.extension_key.data, c_struct.extension_key.len),
    .fallback_supplement = LocaleFallbackSupplement::FromFFI(c_struct.fallback_supplement),
  };
}


#endif // LocaleFallbackConfig_HPP
