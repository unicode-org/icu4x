#ifndef ICU4XTransformResult_HPP
#define ICU4XTransformResult_HPP

#include "ICU4XTransformResult.d.hpp"

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


inline capi::ICU4XTransformResult ICU4XTransformResult::AsFFI() const {
  return static_cast<capi::ICU4XTransformResult>(value);
}

inline ICU4XTransformResult ICU4XTransformResult::FromFFI(capi::ICU4XTransformResult c_enum) {
  switch (c_enum) {
    case capi::ICU4XTransformResult_Modified:
    case capi::ICU4XTransformResult_Unmodified:
      return static_cast<ICU4XTransformResult::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XTransformResult_HPP
