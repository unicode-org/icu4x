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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::BidiDirection BidiDirection::AsFFI() const {
  return static_cast<capi::BidiDirection>(value);
}

inline BidiDirection BidiDirection::FromFFI(capi::BidiDirection c_enum) {
  switch (c_enum) {
    case capi::BidiDirection_Ltr:
    case capi::BidiDirection_Rtl:
    case capi::BidiDirection_Mixed:
      return static_cast<BidiDirection::Value>(c_enum);
    default:
      abort();
  }
}
#endif // BidiDirection_HPP
