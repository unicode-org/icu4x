#ifndef icu4x_LocaleFallbackConfig_HPP
#define icu4x_LocaleFallbackConfig_HPP

#include "LocaleFallbackConfig.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "LocaleFallbackPriority.hpp"
#include "LocaleFallbackSupplement.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::LocaleFallbackConfig icu4x::LocaleFallbackConfig::AsFFI() const {
  return icu4x::capi::LocaleFallbackConfig {
    /* .priority = */ priority.AsFFI(),
    /* .extension_key = */ {extension_key.data(), extension_key.size()},
    /* .fallback_supplement = */ fallback_supplement.AsFFI(),
  };
}

inline icu4x::LocaleFallbackConfig icu4x::LocaleFallbackConfig::FromFFI(icu4x::capi::LocaleFallbackConfig c_struct) {
  return icu4x::LocaleFallbackConfig {
    /* .priority = */ icu4x::LocaleFallbackPriority::FromFFI(c_struct.priority),
    /* .extension_key = */ std::string_view(c_struct.extension_key.data, c_struct.extension_key.len),
    /* .fallback_supplement = */ icu4x::LocaleFallbackSupplement::FromFFI(c_struct.fallback_supplement),
  };
}


#endif // icu4x_LocaleFallbackConfig_HPP
