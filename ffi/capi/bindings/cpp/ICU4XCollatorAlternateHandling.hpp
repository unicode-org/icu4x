#ifndef ICU4XCollatorAlternateHandling_HPP
#define ICU4XCollatorAlternateHandling_HPP

#include "ICU4XCollatorAlternateHandling.d.hpp"

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


inline capi::ICU4XCollatorAlternateHandling ICU4XCollatorAlternateHandling::AsFFI() const {
  return static_cast<capi::ICU4XCollatorAlternateHandling>(value);
}

inline ICU4XCollatorAlternateHandling ICU4XCollatorAlternateHandling::FromFFI(capi::ICU4XCollatorAlternateHandling c_enum) {
  switch (c_enum) {
    case capi::ICU4XCollatorAlternateHandling_Auto:
    case capi::ICU4XCollatorAlternateHandling_NonIgnorable:
    case capi::ICU4XCollatorAlternateHandling_Shifted:
      return static_cast<ICU4XCollatorAlternateHandling::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XCollatorAlternateHandling_HPP
