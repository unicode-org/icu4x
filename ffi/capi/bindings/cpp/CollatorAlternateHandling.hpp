#ifndef CollatorAlternateHandling_HPP
#define CollatorAlternateHandling_HPP

#include "CollatorAlternateHandling.d.hpp"

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


inline capi::CollatorAlternateHandling CollatorAlternateHandling::AsFFI() const {
  return static_cast<capi::CollatorAlternateHandling>(value);
}

inline CollatorAlternateHandling CollatorAlternateHandling::FromFFI(capi::CollatorAlternateHandling c_enum) {
  switch (c_enum) {
    case capi::CollatorAlternateHandling_Auto:
    case capi::CollatorAlternateHandling_NonIgnorable:
    case capi::CollatorAlternateHandling_Shifted:
      return static_cast<CollatorAlternateHandling::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorAlternateHandling_HPP
