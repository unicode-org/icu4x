#ifndef ICU4XCollatorCaseFirst_HPP
#define ICU4XCollatorCaseFirst_HPP

#include "ICU4XCollatorCaseFirst.d.hpp"

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


inline capi::ICU4XCollatorCaseFirst ICU4XCollatorCaseFirst::AsFFI() const {
  return static_cast<capi::ICU4XCollatorCaseFirst>(value);
}

inline ICU4XCollatorCaseFirst ICU4XCollatorCaseFirst::FromFFI(capi::ICU4XCollatorCaseFirst c_enum) {
  switch (c_enum) {
    case capi::ICU4XCollatorCaseFirst_Auto:
    case capi::ICU4XCollatorCaseFirst_Off:
    case capi::ICU4XCollatorCaseFirst_LowerFirst:
    case capi::ICU4XCollatorCaseFirst_UpperFirst:
      return static_cast<ICU4XCollatorCaseFirst::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XCollatorCaseFirst_HPP
