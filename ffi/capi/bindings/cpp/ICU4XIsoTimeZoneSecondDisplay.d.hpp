#ifndef ICU4XIsoTimeZoneSecondDisplay_D_HPP
#define ICU4XIsoTimeZoneSecondDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XIsoTimeZoneSecondDisplay.d.h"


class ICU4XIsoTimeZoneSecondDisplay {
public:
  enum Value {
    Optional = 0,
    Never = 1,
  };

  ICU4XIsoTimeZoneSecondDisplay() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XIsoTimeZoneSecondDisplay(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XIsoTimeZoneSecondDisplay AsFFI() const;
  inline static ICU4XIsoTimeZoneSecondDisplay FromFFI(capi::ICU4XIsoTimeZoneSecondDisplay c_enum);
private:
    Value value;
};


#endif // ICU4XIsoTimeZoneSecondDisplay_D_HPP
