#ifndef ICU4XWeekOf_D_HPP
#define ICU4XWeekOf_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XWeekRelativeUnit.d.hpp"

class ICU4XWeekRelativeUnit;


namespace capi {
    typedef struct ICU4XWeekOf {
      uint16_t week;
      ICU4XWeekRelativeUnit unit;
    } ICU4XWeekOf;
}

struct ICU4XWeekOf {
  uint16_t week;
  ICU4XWeekRelativeUnit unit;

  inline capi::ICU4XWeekOf AsFFI() const;
  inline static ICU4XWeekOf FromFFI(capi::ICU4XWeekOf c_struct);
};


#endif // ICU4XWeekOf_D_HPP
