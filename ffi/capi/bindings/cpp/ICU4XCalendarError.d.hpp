#ifndef ICU4XCalendarError_D_HPP
#define ICU4XCalendarError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCalendarError.d.h"


class ICU4XCalendarError {
public:
  enum Value {
    Unknown = 0,
    OutOfRange = 1,
    UnknownEra = 2,
    UnknownMonthCode = 3,
  };

  ICU4XCalendarError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XCalendarError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XCalendarError AsFFI() const;
  inline static ICU4XCalendarError FromFFI(capi::ICU4XCalendarError c_enum);
private:
    Value value;
};


#endif // ICU4XCalendarError_D_HPP
