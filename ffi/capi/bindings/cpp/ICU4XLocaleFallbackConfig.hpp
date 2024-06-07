#ifndef ICU4XLocaleFallbackConfig_HPP
#define ICU4XLocaleFallbackConfig_HPP

#include "ICU4XLocaleFallbackConfig.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocaleFallbackConfig.h"
#include "ICU4XLocaleFallbackPriority.hpp"
#include "ICU4XLocaleFallbackSupplement.hpp"



inline capi::ICU4XLocaleFallbackConfig ICU4XLocaleFallbackConfig::AsFFI() const {
  return capi::ICU4XLocaleFallbackConfig {
    .priority = priority.AsFFI(),
    .extension_key = { .data = extension_key.data(), .len = extension_key.size() },
    .fallback_supplement = fallback_supplement.AsFFI(),
  };
}

inline ICU4XLocaleFallbackConfig ICU4XLocaleFallbackConfig::FromFFI(capi::ICU4XLocaleFallbackConfig c_struct) {
  return ICU4XLocaleFallbackConfig {
    .priority = ICU4XLocaleFallbackPriority::FromFFI(c_struct.priority),
    .extension_key = std::string_view(c_struct.extension_key.data, c_struct.extension_key.len),
    .fallback_supplement = ICU4XLocaleFallbackSupplement::FromFFI(c_struct.fallback_supplement),
  };
}


#endif // ICU4XLocaleFallbackConfig_HPP
