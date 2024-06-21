#ifndef ICU4XIsoWeekday_HPP
#define ICU4XIsoWeekday_HPP

#include "ICU4XIsoWeekday.d.hpp"

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


inline capi::ICU4XIsoWeekday ICU4XIsoWeekday::AsFFI() const {
  return static_cast<capi::ICU4XIsoWeekday>(value);
}

inline ICU4XIsoWeekday ICU4XIsoWeekday::FromFFI(capi::ICU4XIsoWeekday c_enum) {
  switch (c_enum) {
    case capi::ICU4XIsoWeekday_Monday:
    case capi::ICU4XIsoWeekday_Tuesday:
    case capi::ICU4XIsoWeekday_Wednesday:
    case capi::ICU4XIsoWeekday_Thursday:
    case capi::ICU4XIsoWeekday_Friday:
    case capi::ICU4XIsoWeekday_Saturday:
    case capi::ICU4XIsoWeekday_Sunday:
      return static_cast<ICU4XIsoWeekday::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XIsoWeekday_HPP
