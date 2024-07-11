#ifndef WeekOf_HPP
#define WeekOf_HPP

#include "WeekOf.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "WeekRelativeUnit.hpp"


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::WeekOf WeekOf::AsFFI() const {
  return capi::WeekOf {
    .week = week,
    .unit = unit.AsFFI(),
  };
}

inline WeekOf WeekOf::FromFFI(capi::WeekOf c_struct) {
  return WeekOf {
    .week = c_struct.week,
    .unit = WeekRelativeUnit::FromFFI(c_struct.unit),
  };
}


#endif // WeekOf_HPP
