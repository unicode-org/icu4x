#ifndef ICU4XBidiDirection_HPP
#define ICU4XBidiDirection_HPP

#include "ICU4XBidiDirection.d.hpp"

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


inline capi::ICU4XBidiDirection ICU4XBidiDirection::AsFFI() const {
  return static_cast<capi::ICU4XBidiDirection>(value);
}

inline ICU4XBidiDirection ICU4XBidiDirection::FromFFI(capi::ICU4XBidiDirection c_enum) {
  switch (c_enum) {
    case capi::ICU4XBidiDirection_Ltr:
    case capi::ICU4XBidiDirection_Rtl:
    case capi::ICU4XBidiDirection_Mixed:
      return static_cast<ICU4XBidiDirection::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XBidiDirection_HPP
