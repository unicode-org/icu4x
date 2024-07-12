#ifndef WeekendContainsDay_D_HPP
#define WeekendContainsDay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    struct WeekendContainsDay {
      bool monday;
      bool tuesday;
      bool wednesday;
      bool thursday;
      bool friday;
      bool saturday;
      bool sunday;
    };
} // namespace capi
} // namespace


struct WeekendContainsDay {
  bool monday;
  bool tuesday;
  bool wednesday;
  bool thursday;
  bool friday;
  bool saturday;
  bool sunday;

  inline diplomat::capi::WeekendContainsDay AsFFI() const;
  inline static WeekendContainsDay FromFFI(diplomat::capi::WeekendContainsDay c_struct);
};


#endif // WeekendContainsDay_D_HPP
