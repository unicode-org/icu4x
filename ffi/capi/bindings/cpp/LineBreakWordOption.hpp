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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::LineBreakWordOption LineBreakWordOption::AsFFI() const {
  return static_cast<capi::LineBreakWordOption>(value);
}

inline LineBreakWordOption LineBreakWordOption::FromFFI(capi::LineBreakWordOption c_enum) {
  switch (c_enum) {
    case capi::LineBreakWordOption_Normal:
    case capi::LineBreakWordOption_BreakAll:
    case capi::LineBreakWordOption_KeepAll:
      return static_cast<LineBreakWordOption::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LineBreakWordOption_HPP
