#ifndef ICU4XLineBreakWordOption_HPP
#define ICU4XLineBreakWordOption_HPP

#include "ICU4XLineBreakWordOption.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLineBreakWordOption.h"


inline capi::ICU4XLineBreakWordOption ICU4XLineBreakWordOption::AsFFI() const {
  return static_cast<capi::ICU4XLineBreakWordOption>(value);
}

inline ICU4XLineBreakWordOption ICU4XLineBreakWordOption::FromFFI(capi::ICU4XLineBreakWordOption c_enum) {
  switch (c_enum) {
    case capi::ICU4XLineBreakWordOption_Normal:
    case capi::ICU4XLineBreakWordOption_BreakAll:
    case capi::ICU4XLineBreakWordOption_KeepAll:
      return static_cast<ICU4XLineBreakWordOption::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XLineBreakWordOption_HPP
