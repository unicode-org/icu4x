#ifndef ICU4XFixedDecimalParseError_D_HPP
#define ICU4XFixedDecimalParseError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalParseError.d.h"


class ICU4XFixedDecimalParseError {
public:
  enum Value {
    Unknown = 0,
    Limit = 1,
    Syntax = 2,
  };

  ICU4XFixedDecimalParseError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XFixedDecimalParseError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XFixedDecimalParseError AsFFI() const;
  inline static ICU4XFixedDecimalParseError FromFFI(capi::ICU4XFixedDecimalParseError c_enum);
private:
    Value value;
};


#endif // ICU4XFixedDecimalParseError_D_HPP
