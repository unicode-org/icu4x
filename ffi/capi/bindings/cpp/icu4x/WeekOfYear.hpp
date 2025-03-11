#ifndef icu4x_WeekOfYear_HPP
#define icu4x_WeekOfYear_HPP

#include "WeekOfYear.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace


inline icu4x::capi::WeekOfYear icu4x::WeekOfYear::AsFFI() const {
  return icu4x::capi::WeekOfYear {
    /* .week_number = */ week_number,
    /* .iso_year = */ iso_year,
  };
}

inline icu4x::WeekOfYear icu4x::WeekOfYear::FromFFI(icu4x::capi::WeekOfYear c_struct) {
  return icu4x::WeekOfYear {
    /* .week_number = */ c_struct.week_number,
    /* .iso_year = */ c_struct.iso_year,
  };
}


#endif // icu4x_WeekOfYear_HPP
