#ifndef ICU4XTimeLength_HPP
#define ICU4XTimeLength_HPP

#include "ICU4XTimeLength.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XTimeLength.h"


inline capi::ICU4XTimeLength ICU4XTimeLength::AsFFI() const {
  return static_cast<capi::ICU4XTimeLength>(value);
}

inline ICU4XTimeLength ICU4XTimeLength::FromFFI(capi::ICU4XTimeLength c_enum) {
  switch (c_enum) {
    case capi::ICU4XTimeLength_Full:
    case capi::ICU4XTimeLength_Long:
    case capi::ICU4XTimeLength_Medium:
    case capi::ICU4XTimeLength_Short:
      return static_cast<ICU4XTimeLength::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XTimeLength_HPP
