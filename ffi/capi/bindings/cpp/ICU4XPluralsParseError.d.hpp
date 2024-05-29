#ifndef ICU4XPluralsParseError_D_HPP
#define ICU4XPluralsParseError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XPluralsParseError.d.h"


class ICU4XPluralsParseError {
public:
  enum Value {
    TodoZst = 0,
  };

  ICU4XPluralsParseError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XPluralsParseError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XPluralsParseError AsFFI() const;
  inline static ICU4XPluralsParseError FromFFI(capi::ICU4XPluralsParseError c_enum);
private:
    Value value;
};


#endif // ICU4XPluralsParseError_D_HPP
