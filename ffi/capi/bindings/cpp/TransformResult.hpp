#ifndef TransformResult_HPP
#define TransformResult_HPP

#include "TransformResult.d.hpp"

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


inline capi::TransformResult TransformResult::AsFFI() const {
  return static_cast<capi::TransformResult>(value);
}

inline TransformResult TransformResult::FromFFI(capi::TransformResult c_enum) {
  switch (c_enum) {
    case capi::TransformResult_Modified:
    case capi::TransformResult_Unmodified:
      return static_cast<TransformResult::Value>(c_enum);
    default:
      abort();
  }
}
#endif // TransformResult_HPP
