#ifndef BidiDirection_HPP
#define BidiDirection_HPP

#include "BidiDirection.d.hpp"

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

inline diplomat::capi::BidiDirection BidiDirection::AsFFI() const {
  return static_cast<diplomat::capi::BidiDirection>(value);
}

inline BidiDirection BidiDirection::FromFFI(diplomat::capi::BidiDirection c_enum) {
  switch (c_enum) {
    case diplomat::capi::BidiDirection_Ltr:
    case diplomat::capi::BidiDirection_Rtl:
    case diplomat::capi::BidiDirection_Mixed:
      return static_cast<BidiDirection::Value>(c_enum);
    default:
      abort();
  }
}
#endif // BidiDirection_HPP
