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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::LineBreakStrictness LineBreakStrictness::AsFFI() const {
  return static_cast<diplomat::capi::LineBreakStrictness>(value);
}

inline LineBreakStrictness LineBreakStrictness::FromFFI(diplomat::capi::LineBreakStrictness c_enum) {
  switch (c_enum) {
    case diplomat::capi::LineBreakStrictness_Loose:
    case diplomat::capi::LineBreakStrictness_Normal:
    case diplomat::capi::LineBreakStrictness_Strict:
    case diplomat::capi::LineBreakStrictness_Anywhere:
      return static_cast<LineBreakStrictness::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LineBreakStrictness_HPP
