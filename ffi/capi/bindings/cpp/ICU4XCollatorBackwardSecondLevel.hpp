#ifndef ICU4XCollatorBackwardSecondLevel_HPP
#define ICU4XCollatorBackwardSecondLevel_HPP

#include "ICU4XCollatorBackwardSecondLevel.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollatorBackwardSecondLevel.h"


inline capi::ICU4XCollatorBackwardSecondLevel ICU4XCollatorBackwardSecondLevel::AsFFI() const {
  return static_cast<capi::ICU4XCollatorBackwardSecondLevel>(value);
}

inline ICU4XCollatorBackwardSecondLevel ICU4XCollatorBackwardSecondLevel::FromFFI(capi::ICU4XCollatorBackwardSecondLevel c_enum) {
  switch (c_enum) {
    case capi::ICU4XCollatorBackwardSecondLevel_Auto:
    case capi::ICU4XCollatorBackwardSecondLevel_Off:
    case capi::ICU4XCollatorBackwardSecondLevel_On:
      return static_cast<ICU4XCollatorBackwardSecondLevel::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XCollatorBackwardSecondLevel_HPP
