#ifndef CollatorStrength_HPP
#define CollatorStrength_HPP

#include "CollatorStrength.d.hpp"

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

inline diplomat::capi::CollatorStrength CollatorStrength::AsFFI() const {
  return static_cast<diplomat::capi::CollatorStrength>(value);
}

inline CollatorStrength CollatorStrength::FromFFI(diplomat::capi::CollatorStrength c_enum) {
  switch (c_enum) {
    case diplomat::capi::CollatorStrength_Auto:
    case diplomat::capi::CollatorStrength_Primary:
    case diplomat::capi::CollatorStrength_Secondary:
    case diplomat::capi::CollatorStrength_Tertiary:
    case diplomat::capi::CollatorStrength_Quaternary:
    case diplomat::capi::CollatorStrength_Identical:
      return static_cast<CollatorStrength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorStrength_HPP
