#ifndef ICU4XCollatorNumeric_HPP
#define ICU4XCollatorNumeric_HPP

#include "ICU4XCollatorNumeric.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollatorNumeric.h"


inline capi::ICU4XCollatorNumeric ICU4XCollatorNumeric::AsFFI() const {
  return static_cast<capi::ICU4XCollatorNumeric>(value);
}

inline ICU4XCollatorNumeric ICU4XCollatorNumeric::FromFFI(capi::ICU4XCollatorNumeric c_enum) {
  switch (c_enum) {
    case capi::ICU4XCollatorNumeric_Auto:
    case capi::ICU4XCollatorNumeric_Off:
    case capi::ICU4XCollatorNumeric_On:
      return static_cast<ICU4XCollatorNumeric::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XCollatorNumeric_HPP
