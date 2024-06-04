#ifndef ICU4XFixedDecimalSign_D_HPP
#define ICU4XFixedDecimalSign_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalSign.d.h"


class ICU4XFixedDecimalSign {
public:
  enum Value {
    None = 0,
    Negative = 1,
    Positive = 2,
  };

  ICU4XFixedDecimalSign() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XFixedDecimalSign(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XFixedDecimalSign AsFFI() const;
  inline static ICU4XFixedDecimalSign FromFFI(capi::ICU4XFixedDecimalSign c_enum);
private:
    Value value;
};


#endif // ICU4XFixedDecimalSign_D_HPP
