#ifndef ICU4XWeekOf_HPP
#define ICU4XWeekOf_HPP

#include "ICU4XWeekOf.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XWeekOf.h"
#include "ICU4XWeekRelativeUnit.hpp"



inline capi::ICU4XWeekOf ICU4XWeekOf::AsFFI() const {
  return capi::ICU4XWeekOf {
    .week = week,
    .unit = unit.AsFFI(),
  };
}

inline ICU4XWeekOf ICU4XWeekOf::FromFFI(capi::ICU4XWeekOf c_struct) {
  return ICU4XWeekOf {
    .week = c_struct.week,
    .unit = ICU4XWeekRelativeUnit::FromFFI(c_struct.unit),
  };
}


#endif // ICU4XWeekOf_HPP
