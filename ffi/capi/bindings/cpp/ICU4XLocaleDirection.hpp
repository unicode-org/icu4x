#ifndef ICU4XLocaleDirection_HPP
#define ICU4XLocaleDirection_HPP

#include "ICU4XLocaleDirection.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocaleDirection.h"


inline capi::ICU4XLocaleDirection ICU4XLocaleDirection::AsFFI() const {
  return static_cast<capi::ICU4XLocaleDirection>(value);
}

inline ICU4XLocaleDirection ICU4XLocaleDirection::FromFFI(capi::ICU4XLocaleDirection c_enum) {
  switch (c_enum) {
    case capi::ICU4XLocaleDirection_LeftToRight:
    case capi::ICU4XLocaleDirection_RightToLeft:
    case capi::ICU4XLocaleDirection_Unknown:
      return static_cast<ICU4XLocaleDirection::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XLocaleDirection_HPP
