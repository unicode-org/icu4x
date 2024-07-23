#ifndef WeekOf_HPP
#define WeekOf_HPP

#include "WeekOf.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "WeekRelativeUnit.hpp"
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline diplomat::capi::WeekOf WeekOf::AsFFI() const {
  return diplomat::capi::WeekOf {
    /* .week = */ week,
    /* .unit = */ unit.AsFFI(),
  };
}

inline WeekOf WeekOf::FromFFI(diplomat::capi::WeekOf c_struct) {
  return WeekOf {
    /* .week = */ c_struct.week,
    /* .unit = */ WeekRelativeUnit::FromFFI(c_struct.unit),
  };
}


#endif // WeekOf_HPP
