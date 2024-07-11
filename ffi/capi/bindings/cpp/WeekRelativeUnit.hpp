#ifndef WeekRelativeUnit_HPP
#define WeekRelativeUnit_HPP

#include "WeekRelativeUnit.d.hpp"

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


inline capi::WeekRelativeUnit WeekRelativeUnit::AsFFI() const {
  return static_cast<capi::WeekRelativeUnit>(value);
}

inline WeekRelativeUnit WeekRelativeUnit::FromFFI(capi::WeekRelativeUnit c_enum) {
  switch (c_enum) {
    case capi::WeekRelativeUnit_Previous:
    case capi::WeekRelativeUnit_Current:
    case capi::WeekRelativeUnit_Next:
      return static_cast<WeekRelativeUnit::Value>(c_enum);
    default:
      abort();
  }
}
#endif // WeekRelativeUnit_HPP
