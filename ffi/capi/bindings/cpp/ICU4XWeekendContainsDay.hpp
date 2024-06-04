#ifndef ICU4XWeekendContainsDay_HPP
#define ICU4XWeekendContainsDay_HPP

#include "ICU4XWeekendContainsDay.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XWeekendContainsDay.h"



inline capi::ICU4XWeekendContainsDay ICU4XWeekendContainsDay::AsFFI() const {
  return capi::ICU4XWeekendContainsDay {
    .monday = monday,
    .tuesday = tuesday,
    .wednesday = wednesday,
    .thursday = thursday,
    .friday = friday,
    .saturday = saturday,
    .sunday = sunday,
  };
}

inline ICU4XWeekendContainsDay ICU4XWeekendContainsDay::FromFFI(capi::ICU4XWeekendContainsDay c_struct) {
  return ICU4XWeekendContainsDay {
    .monday = c_struct.monday,
    .tuesday = c_struct.tuesday,
    .wednesday = c_struct.wednesday,
    .thursday = c_struct.thursday,
    .friday = c_struct.friday,
    .saturday = c_struct.saturday,
    .sunday = c_struct.sunday,
  };
}


#endif // ICU4XWeekendContainsDay_HPP
