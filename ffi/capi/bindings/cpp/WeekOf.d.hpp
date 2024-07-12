#ifndef WeekOf_D_HPP
#define WeekOf_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "WeekRelativeUnit.d.hpp"

class WeekRelativeUnit;


namespace diplomat {
namespace capi {
    struct WeekOf {
      uint16_t week;
      diplomat::capi::WeekRelativeUnit unit;
    };
} // namespace capi
} // namespace


struct WeekOf {
  uint16_t week;
  WeekRelativeUnit unit;

  inline diplomat::capi::WeekOf AsFFI() const;
  inline static WeekOf FromFFI(diplomat::capi::WeekOf c_struct);
};


#endif // WeekOf_D_HPP
