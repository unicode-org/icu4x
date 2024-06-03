#ifndef ICU4XDisplayNamesFallback_HPP
#define ICU4XDisplayNamesFallback_HPP

#include "ICU4XDisplayNamesFallback.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDisplayNamesFallback.h"


inline capi::ICU4XDisplayNamesFallback ICU4XDisplayNamesFallback::AsFFI() const {
  return static_cast<capi::ICU4XDisplayNamesFallback>(value);
}

inline ICU4XDisplayNamesFallback ICU4XDisplayNamesFallback::FromFFI(capi::ICU4XDisplayNamesFallback c_enum) {
  switch (c_enum) {
    case capi::ICU4XDisplayNamesFallback_Code:
    case capi::ICU4XDisplayNamesFallback_None:
      return static_cast<ICU4XDisplayNamesFallback::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XDisplayNamesFallback_HPP
