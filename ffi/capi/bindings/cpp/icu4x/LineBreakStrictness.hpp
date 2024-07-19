#ifndef icu4x_LineBreakStrictness_HPP
#define icu4x_LineBreakStrictness_HPP

#include "LineBreakStrictness.d.hpp"

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

inline icu4x::capi::LineBreakStrictness icu4x::LineBreakStrictness::AsFFI() const {
  return static_cast<icu4x::capi::LineBreakStrictness>(value);
}

inline icu4x::LineBreakStrictness icu4x::LineBreakStrictness::FromFFI(icu4x::capi::LineBreakStrictness c_enum) {
  switch (c_enum) {
    case icu4x::capi::LineBreakStrictness_Loose:
    case icu4x::capi::LineBreakStrictness_Normal:
    case icu4x::capi::LineBreakStrictness_Strict:
    case icu4x::capi::LineBreakStrictness_Anywhere:
      return static_cast<icu4x::LineBreakStrictness::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_LineBreakStrictness_HPP
