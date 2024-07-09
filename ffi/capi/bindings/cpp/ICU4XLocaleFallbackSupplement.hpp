#ifndef ICU4XLocaleFallbackSupplement_HPP
#define ICU4XLocaleFallbackSupplement_HPP

#include "ICU4XLocaleFallbackSupplement.d.hpp"

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


inline capi::ICU4XLocaleFallbackSupplement ICU4XLocaleFallbackSupplement::AsFFI() const {
  return static_cast<capi::ICU4XLocaleFallbackSupplement>(value);
}

inline ICU4XLocaleFallbackSupplement ICU4XLocaleFallbackSupplement::FromFFI(capi::ICU4XLocaleFallbackSupplement c_enum) {
  switch (c_enum) {
    case capi::ICU4XLocaleFallbackSupplement_None:
    case capi::ICU4XLocaleFallbackSupplement_Collation:
      return static_cast<ICU4XLocaleFallbackSupplement::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XLocaleFallbackSupplement_HPP
