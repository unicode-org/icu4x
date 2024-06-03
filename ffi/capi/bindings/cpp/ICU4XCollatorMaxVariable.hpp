#ifndef ICU4XCollatorMaxVariable_HPP
#define ICU4XCollatorMaxVariable_HPP

#include "ICU4XCollatorMaxVariable.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollatorMaxVariable.h"


inline capi::ICU4XCollatorMaxVariable ICU4XCollatorMaxVariable::AsFFI() const {
  return static_cast<capi::ICU4XCollatorMaxVariable>(value);
}

inline ICU4XCollatorMaxVariable ICU4XCollatorMaxVariable::FromFFI(capi::ICU4XCollatorMaxVariable c_enum) {
  switch (c_enum) {
    case capi::ICU4XCollatorMaxVariable_Auto:
    case capi::ICU4XCollatorMaxVariable_Space:
    case capi::ICU4XCollatorMaxVariable_Punctuation:
    case capi::ICU4XCollatorMaxVariable_Symbol:
    case capi::ICU4XCollatorMaxVariable_Currency:
      return static_cast<ICU4XCollatorMaxVariable::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XCollatorMaxVariable_HPP
