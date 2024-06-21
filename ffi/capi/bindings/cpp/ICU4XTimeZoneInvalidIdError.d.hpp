#ifndef ICU4XTimeZoneInvalidIdError_D_HPP
#define ICU4XTimeZoneInvalidIdError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XTimeZoneInvalidIdError {
      ICU4XTimeZoneInvalidIdError_TodoZst = 0,
    } ICU4XTimeZoneInvalidIdError;
}

class ICU4XTimeZoneInvalidIdError {
public:
  enum Value {
    TodoZst = 0,
  };

  ICU4XTimeZoneInvalidIdError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XTimeZoneInvalidIdError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XTimeZoneInvalidIdError AsFFI() const;
  inline static ICU4XTimeZoneInvalidIdError FromFFI(capi::ICU4XTimeZoneInvalidIdError c_enum);
private:
    Value value;
};


#endif // ICU4XTimeZoneInvalidIdError_D_HPP
