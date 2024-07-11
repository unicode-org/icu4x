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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::TrailingCase TrailingCase::AsFFI() const {
  return static_cast<capi::TrailingCase>(value);
}

inline TrailingCase TrailingCase::FromFFI(capi::TrailingCase c_enum) {
  switch (c_enum) {
    case capi::TrailingCase_Lower:
    case capi::TrailingCase_Unchanged:
      return static_cast<TrailingCase::Value>(c_enum);
    default:
      abort();
  }
}
#endif // TrailingCase_HPP
