#ifndef ICU4XTrailingCase_HPP
#define ICU4XTrailingCase_HPP

#include "ICU4XTrailingCase.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XTrailingCase.h"


inline capi::ICU4XTrailingCase ICU4XTrailingCase::AsFFI() const {
  return static_cast<capi::ICU4XTrailingCase>(value);
}

inline ICU4XTrailingCase ICU4XTrailingCase::FromFFI(capi::ICU4XTrailingCase c_enum) {
  switch (c_enum) {
    case capi::ICU4XTrailingCase_Lower:
    case capi::ICU4XTrailingCase_Unchanged:
      return static_cast<ICU4XTrailingCase::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XTrailingCase_HPP
