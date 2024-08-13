#ifndef icu4x_LineBreakWordOption_HPP
#define icu4x_LineBreakWordOption_HPP

#include "LineBreakWordOption.d.hpp"

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

inline icu4x::capi::LineBreakWordOption icu4x::LineBreakWordOption::AsFFI() const {
  return static_cast<icu4x::capi::LineBreakWordOption>(value);
}

inline icu4x::LineBreakWordOption icu4x::LineBreakWordOption::FromFFI(icu4x::capi::LineBreakWordOption c_enum) {
  switch (c_enum) {
    case icu4x::capi::LineBreakWordOption_Normal:
    case icu4x::capi::LineBreakWordOption_BreakAll:
    case icu4x::capi::LineBreakWordOption_KeepAll:
      return static_cast<icu4x::LineBreakWordOption::Value>(c_enum);
    default:
      abort();
  }
}
#endif // icu4x_LineBreakWordOption_HPP
