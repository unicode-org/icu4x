#ifndef ICU4XFixedDecimalRoundingIncrement_D_HPP
#define ICU4XFixedDecimalRoundingIncrement_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XFixedDecimalRoundingIncrement.d.h"


class ICU4XFixedDecimalRoundingIncrement {
public:
  enum Value {
    MultiplesOf1 = 0,
    MultiplesOf2 = 1,
    MultiplesOf5 = 2,
    MultiplesOf25 = 3,
  };

  ICU4XFixedDecimalRoundingIncrement() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XFixedDecimalRoundingIncrement(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XFixedDecimalRoundingIncrement AsFFI() const;
  inline static ICU4XFixedDecimalRoundingIncrement FromFFI(capi::ICU4XFixedDecimalRoundingIncrement c_enum);
private:
    Value value;
};


#endif // ICU4XFixedDecimalRoundingIncrement_D_HPP
