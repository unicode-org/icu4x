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


namespace capi {
    typedef struct WeekOf {
      uint16_t week;
      WeekRelativeUnit unit;
    } WeekOf;
}

struct WeekOf {
  uint16_t week;
  WeekRelativeUnit unit;

  inline capi::WeekOf AsFFI() const;
  inline static WeekOf FromFFI(capi::WeekOf c_struct);
};


#endif // WeekOf_D_HPP
