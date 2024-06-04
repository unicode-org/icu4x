#ifndef ICU4XWeekendContainsDay_D_HPP
#define ICU4XWeekendContainsDay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XWeekendContainsDay.d.h"


struct ICU4XWeekendContainsDay {
  bool monday;
  bool tuesday;
  bool wednesday;
  bool thursday;
  bool friday;
  bool saturday;
  bool sunday;

  inline capi::ICU4XWeekendContainsDay AsFFI() const;
  inline static ICU4XWeekendContainsDay FromFFI(capi::ICU4XWeekendContainsDay c_struct);
};


#endif // ICU4XWeekendContainsDay_D_HPP
