#ifndef CalendarParseError_D_HPP
#define CalendarParseError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum CalendarParseError {
      CalendarParseError_Unknown = 0,
      CalendarParseError_InvalidSyntax = 1,
      CalendarParseError_OutOfRange = 2,
      CalendarParseError_MissingFields = 3,
      CalendarParseError_UnknownCalendar = 4,
    };
} // namespace capi
} // namespace

class CalendarParseError {
public:
  enum Value {
    Unknown = 0,
    InvalidSyntax = 1,
    OutOfRange = 2,
    MissingFields = 3,
    UnknownCalendar = 4,
  };

  CalendarParseError() = default;
  // Implicit conversions between enum and ::Value
  constexpr CalendarParseError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::CalendarParseError AsFFI() const;
  inline static CalendarParseError FromFFI(diplomat::capi::CalendarParseError c_enum);
private:
    Value value;
};


#endif // CalendarParseError_D_HPP
