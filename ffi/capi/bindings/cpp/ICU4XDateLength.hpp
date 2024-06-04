#ifndef ICU4XDateLength_HPP
#define ICU4XDateLength_HPP

#include "ICU4XDateLength.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDateLength.h"


inline capi::ICU4XDateLength ICU4XDateLength::AsFFI() const {
  return static_cast<capi::ICU4XDateLength>(value);
}

inline ICU4XDateLength ICU4XDateLength::FromFFI(capi::ICU4XDateLength c_enum) {
  switch (c_enum) {
    case capi::ICU4XDateLength_Full:
    case capi::ICU4XDateLength_Long:
    case capi::ICU4XDateLength_Medium:
    case capi::ICU4XDateLength_Short:
      return static_cast<ICU4XDateLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XDateLength_HPP
