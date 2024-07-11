#ifndef LineBreakStrictness_HPP
#define LineBreakStrictness_HPP

#include "LineBreakStrictness.d.hpp"

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


inline capi::LineBreakStrictness LineBreakStrictness::AsFFI() const {
  return static_cast<capi::LineBreakStrictness>(value);
}

inline LineBreakStrictness LineBreakStrictness::FromFFI(capi::LineBreakStrictness c_enum) {
  switch (c_enum) {
    case capi::LineBreakStrictness_Loose:
    case capi::LineBreakStrictness_Normal:
    case capi::LineBreakStrictness_Strict:
    case capi::LineBreakStrictness_Anywhere:
      return static_cast<LineBreakStrictness::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LineBreakStrictness_HPP
