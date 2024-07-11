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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::CollatorStrength CollatorStrength::AsFFI() const {
  return static_cast<capi::CollatorStrength>(value);
}

inline CollatorStrength CollatorStrength::FromFFI(capi::CollatorStrength c_enum) {
  switch (c_enum) {
    case capi::CollatorStrength_Auto:
    case capi::CollatorStrength_Primary:
    case capi::CollatorStrength_Secondary:
    case capi::CollatorStrength_Tertiary:
    case capi::CollatorStrength_Quaternary:
    case capi::CollatorStrength_Identical:
      return static_cast<CollatorStrength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorStrength_HPP
