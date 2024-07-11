#ifndef WeekendContainsDay_D_HPP
#define WeekendContainsDay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef struct WeekendContainsDay {
      bool monday;
      bool tuesday;
      bool wednesday;
      bool thursday;
      bool friday;
      bool saturday;
      bool sunday;
    } WeekendContainsDay;
}

struct WeekendContainsDay {
  bool monday;
  bool tuesday;
  bool wednesday;
  bool thursday;
  bool friday;
  bool saturday;
  bool sunday;

  inline capi::WeekendContainsDay AsFFI() const;
  inline static WeekendContainsDay FromFFI(capi::WeekendContainsDay c_struct);
};


#endif // WeekendContainsDay_D_HPP
