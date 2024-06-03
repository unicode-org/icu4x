#ifndef ICU4XFixedDecimalSign_HPP
#define ICU4XFixedDecimalSign_HPP

#include "ICU4XFixedDecimalSign.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalSign.h"


inline capi::ICU4XFixedDecimalSign ICU4XFixedDecimalSign::AsFFI() const {
  return static_cast<capi::ICU4XFixedDecimalSign>(value);
}

inline ICU4XFixedDecimalSign ICU4XFixedDecimalSign::FromFFI(capi::ICU4XFixedDecimalSign c_enum) {
  switch (c_enum) {
    case capi::ICU4XFixedDecimalSign_None:
    case capi::ICU4XFixedDecimalSign_Negative:
    case capi::ICU4XFixedDecimalSign_Positive:
      return static_cast<ICU4XFixedDecimalSign::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XFixedDecimalSign_HPP
