#ifndef ICU4XCollatorStrength_HPP
#define ICU4XCollatorStrength_HPP

#include "ICU4XCollatorStrength.d.hpp"

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


inline capi::ICU4XCollatorStrength ICU4XCollatorStrength::AsFFI() const {
  return static_cast<capi::ICU4XCollatorStrength>(value);
}

inline ICU4XCollatorStrength ICU4XCollatorStrength::FromFFI(capi::ICU4XCollatorStrength c_enum) {
  switch (c_enum) {
    case capi::ICU4XCollatorStrength_Auto:
    case capi::ICU4XCollatorStrength_Primary:
    case capi::ICU4XCollatorStrength_Secondary:
    case capi::ICU4XCollatorStrength_Tertiary:
    case capi::ICU4XCollatorStrength_Quaternary:
    case capi::ICU4XCollatorStrength_Identical:
      return static_cast<ICU4XCollatorStrength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XCollatorStrength_HPP
