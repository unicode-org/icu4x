#ifndef ICU4XWeekRelativeUnit_D_HPP
#define ICU4XWeekRelativeUnit_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XWeekRelativeUnit.d.h"


class ICU4XWeekRelativeUnit {
public:
  enum Value {
    Previous = 0,
    Current = 1,
    Next = 2,
  };

  ICU4XWeekRelativeUnit() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XWeekRelativeUnit(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XWeekRelativeUnit AsFFI() const;
  inline static ICU4XWeekRelativeUnit FromFFI(capi::ICU4XWeekRelativeUnit c_enum);
private:
    Value value;
};


#endif // ICU4XWeekRelativeUnit_D_HPP
