#ifndef LineBreakWordOption_HPP
#define LineBreakWordOption_HPP

#include "LineBreakWordOption.d.hpp"

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

inline diplomat::capi::LineBreakWordOption LineBreakWordOption::AsFFI() const {
  return static_cast<diplomat::capi::LineBreakWordOption>(value);
}

inline LineBreakWordOption LineBreakWordOption::FromFFI(diplomat::capi::LineBreakWordOption c_enum) {
  switch (c_enum) {
    case diplomat::capi::LineBreakWordOption_Normal:
    case diplomat::capi::LineBreakWordOption_BreakAll:
    case diplomat::capi::LineBreakWordOption_KeepAll:
      return static_cast<LineBreakWordOption::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LineBreakWordOption_HPP
