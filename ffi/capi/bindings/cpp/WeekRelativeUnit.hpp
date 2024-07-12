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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::WeekRelativeUnit WeekRelativeUnit::AsFFI() const {
  return static_cast<diplomat::capi::WeekRelativeUnit>(value);
}

inline WeekRelativeUnit WeekRelativeUnit::FromFFI(diplomat::capi::WeekRelativeUnit c_enum) {
  switch (c_enum) {
    case diplomat::capi::WeekRelativeUnit_Previous:
    case diplomat::capi::WeekRelativeUnit_Current:
    case diplomat::capi::WeekRelativeUnit_Next:
      return static_cast<WeekRelativeUnit::Value>(c_enum);
    default:
      abort();
  }
}
#endif // WeekRelativeUnit_HPP
