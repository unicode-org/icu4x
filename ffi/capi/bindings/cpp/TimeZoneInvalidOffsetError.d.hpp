#ifndef TimeZoneInvalidOffsetError_D_HPP
#define TimeZoneInvalidOffsetError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum TimeZoneInvalidOffsetError {
      TimeZoneInvalidOffsetError_TodoZst = 0,
    } TimeZoneInvalidOffsetError;
}

class TimeZoneInvalidOffsetError {
public:
  enum Value {
    TodoZst = 0,
  };

  TimeZoneInvalidOffsetError() = default;
  // Implicit conversions between enum and ::Value
  constexpr TimeZoneInvalidOffsetError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::TimeZoneInvalidOffsetError AsFFI() const;
  inline static TimeZoneInvalidOffsetError FromFFI(capi::TimeZoneInvalidOffsetError c_enum);
private:
    Value value;
};


#endif // TimeZoneInvalidOffsetError_D_HPP
