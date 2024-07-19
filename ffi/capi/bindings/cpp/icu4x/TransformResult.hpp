#ifndef icu4x_TransformResult_HPP
#define icu4x_TransformResult_HPP

#include "TransformResult.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline icu4x::capi::TransformResult icu4x::TransformResult::AsFFI() const {
  return static_cast<icu4x::capi::TransformResult>(value);
}

inline icu4x::TransformResult icu4x::TransformResult::FromFFI(icu4x::capi::TransformResult c_enum) {
  switch (c_enum) {
    case icu4x::capi::TransformResult_Modified:
    case icu4x::capi::TransformResult_Unmodified:
      return static_cast<icu4x::TransformResult::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_TransformResult_HPP
