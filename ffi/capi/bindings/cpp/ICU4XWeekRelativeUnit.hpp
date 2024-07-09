#ifndef ICU4XWeekRelativeUnit_HPP
#define ICU4XWeekRelativeUnit_HPP

#include "ICU4XWeekRelativeUnit.d.hpp"

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


inline capi::ICU4XWeekRelativeUnit ICU4XWeekRelativeUnit::AsFFI() const {
  return static_cast<capi::ICU4XWeekRelativeUnit>(value);
}

inline ICU4XWeekRelativeUnit ICU4XWeekRelativeUnit::FromFFI(capi::ICU4XWeekRelativeUnit c_enum) {
  switch (c_enum) {
    case capi::ICU4XWeekRelativeUnit_Previous:
    case capi::ICU4XWeekRelativeUnit_Current:
    case capi::ICU4XWeekRelativeUnit_Next:
      return static_cast<ICU4XWeekRelativeUnit::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XWeekRelativeUnit_HPP
