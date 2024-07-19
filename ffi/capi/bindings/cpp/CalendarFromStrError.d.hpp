#ifndef CalendarFromStrError_D_HPP
#define CalendarFromStrError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum CalendarFromStrError {
      CalendarFromStrError_Unknown = 0,
      CalendarFromStrError_InvalidSyntax = 1,
      CalendarFromStrError_OutOfRange = 2,
      CalendarFromStrError_MissingFields = 3,
      CalendarFromStrError_UnknownCalendar = 4,
    };
} // namespace capi
} // namespace

class CalendarFromStrError {
public:
  enum Value {
    Unknown = 0,
    InvalidSyntax = 1,
    OutOfRange = 2,
    MissingFields = 3,
    UnknownCalendar = 4,
  };

  CalendarFromStrError() = default;
  // Implicit conversions between enum and ::Value
  constexpr CalendarFromStrError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::CalendarFromStrError AsFFI() const;
  inline static CalendarFromStrError FromFFI(diplomat::capi::CalendarFromStrError c_enum);
private:
    Value value;
};


#endif // CalendarFromStrError_D_HPP
