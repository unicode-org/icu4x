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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::IsoWeekday IsoWeekday::AsFFI() const {
  return static_cast<diplomat::capi::IsoWeekday>(value);
}

inline IsoWeekday IsoWeekday::FromFFI(diplomat::capi::IsoWeekday c_enum) {
  switch (c_enum) {
    case diplomat::capi::IsoWeekday_Monday:
    case diplomat::capi::IsoWeekday_Tuesday:
    case diplomat::capi::IsoWeekday_Wednesday:
    case diplomat::capi::IsoWeekday_Thursday:
    case diplomat::capi::IsoWeekday_Friday:
    case diplomat::capi::IsoWeekday_Saturday:
    case diplomat::capi::IsoWeekday_Sunday:
      return static_cast<IsoWeekday::Value>(c_enum);
    default:
      abort();
  }
}
#endif // IsoWeekday_HPP
