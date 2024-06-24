#ifndef ICU4XCollatorCaseLevel_HPP
#define ICU4XCollatorCaseLevel_HPP

#include "ICU4XCollatorCaseLevel.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::ICU4XCollatorCaseLevel ICU4XCollatorCaseLevel::AsFFI() const {
  return static_cast<capi::ICU4XCollatorCaseLevel>(value);
}

inline ICU4XCollatorCaseLevel ICU4XCollatorCaseLevel::FromFFI(capi::ICU4XCollatorCaseLevel c_enum) {
  switch (c_enum) {
    case capi::ICU4XCollatorCaseLevel_Auto:
    case capi::ICU4XCollatorCaseLevel_Off:
    case capi::ICU4XCollatorCaseLevel_On:
      return static_cast<ICU4XCollatorCaseLevel::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XCollatorCaseLevel_HPP
