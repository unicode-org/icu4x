#ifndef IsoWeekday_HPP
#define IsoWeekday_HPP

#include "IsoWeekday.d.hpp"

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


inline capi::IsoWeekday IsoWeekday::AsFFI() const {
  return static_cast<capi::IsoWeekday>(value);
}

inline IsoWeekday IsoWeekday::FromFFI(capi::IsoWeekday c_enum) {
  switch (c_enum) {
    case capi::IsoWeekday_Monday:
    case capi::IsoWeekday_Tuesday:
    case capi::IsoWeekday_Wednesday:
    case capi::IsoWeekday_Thursday:
    case capi::IsoWeekday_Friday:
    case capi::IsoWeekday_Saturday:
    case capi::IsoWeekday_Sunday:
      return static_cast<IsoWeekday::Value>(c_enum);
    default:
      abort();
  }
}
#endif // IsoWeekday_HPP
