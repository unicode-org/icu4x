#ifndef TrailingCase_HPP
#define TrailingCase_HPP

#include "TrailingCase.d.hpp"

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

inline diplomat::capi::TrailingCase TrailingCase::AsFFI() const {
  return static_cast<diplomat::capi::TrailingCase>(value);
}

inline TrailingCase TrailingCase::FromFFI(diplomat::capi::TrailingCase c_enum) {
  switch (c_enum) {
    case diplomat::capi::TrailingCase_Lower:
    case diplomat::capi::TrailingCase_Unchanged:
      return static_cast<TrailingCase::Value>(c_enum);
    default:
      abort();
  }
}
#endif // TrailingCase_HPP
