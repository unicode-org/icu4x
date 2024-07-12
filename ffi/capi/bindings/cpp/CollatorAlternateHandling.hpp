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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::CollatorAlternateHandling CollatorAlternateHandling::AsFFI() const {
  return static_cast<diplomat::capi::CollatorAlternateHandling>(value);
}

inline CollatorAlternateHandling CollatorAlternateHandling::FromFFI(diplomat::capi::CollatorAlternateHandling c_enum) {
  switch (c_enum) {
    case diplomat::capi::CollatorAlternateHandling_Auto:
    case diplomat::capi::CollatorAlternateHandling_NonIgnorable:
    case diplomat::capi::CollatorAlternateHandling_Shifted:
      return static_cast<CollatorAlternateHandling::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CollatorAlternateHandling_HPP
