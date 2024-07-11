#ifndef CalendarError_D_HPP
#define CalendarError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum CalendarError {
      CalendarError_Unknown = 0,
      CalendarError_OutOfRange = 1,
      CalendarError_UnknownEra = 2,
      CalendarError_UnknownMonthCode = 3,
    } CalendarError;
} // namespace capi
} // namespace

class CalendarError {
public:
  enum Value {
    Unknown = 0,
    OutOfRange = 1,
    UnknownEra = 2,
    UnknownMonthCode = 3,
  };

  CalendarError() = default;
  // Implicit conversions between enum and ::Value
  constexpr CalendarError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::CalendarError AsFFI() const;
  inline static CalendarError FromFFI(diplomat::capi::CalendarError c_enum);
private:
    Value value;
};


#endif // CalendarError_D_HPP
