#ifndef ICU4XTimeZoneInvalidOffsetError_D_HPP
#define ICU4XTimeZoneInvalidOffsetError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XTimeZoneInvalidOffsetError {
      ICU4XTimeZoneInvalidOffsetError_TodoZst = 0,
    } ICU4XTimeZoneInvalidOffsetError;
}

class ICU4XTimeZoneInvalidOffsetError {
public:
  enum Value {
    TodoZst = 0,
  };

  ICU4XTimeZoneInvalidOffsetError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XTimeZoneInvalidOffsetError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XTimeZoneInvalidOffsetError AsFFI() const;
  inline static ICU4XTimeZoneInvalidOffsetError FromFFI(capi::ICU4XTimeZoneInvalidOffsetError c_enum);
private:
    Value value;
};


#endif // ICU4XTimeZoneInvalidOffsetError_D_HPP
