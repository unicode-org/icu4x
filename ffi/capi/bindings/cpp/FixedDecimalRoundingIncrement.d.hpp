#ifndef FixedDecimalRoundingIncrement_D_HPP
#define FixedDecimalRoundingIncrement_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum FixedDecimalRoundingIncrement {
      FixedDecimalRoundingIncrement_MultiplesOf1 = 0,
      FixedDecimalRoundingIncrement_MultiplesOf2 = 1,
      FixedDecimalRoundingIncrement_MultiplesOf5 = 2,
      FixedDecimalRoundingIncrement_MultiplesOf25 = 3,
    };
} // namespace capi
} // namespace

class FixedDecimalRoundingIncrement {
public:
  enum Value {
    MultiplesOf1 = 0,
    MultiplesOf2 = 1,
    MultiplesOf5 = 2,
    MultiplesOf25 = 3,
  };

  FixedDecimalRoundingIncrement() = default;
  // Implicit conversions between enum and ::Value
  constexpr FixedDecimalRoundingIncrement(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::FixedDecimalRoundingIncrement AsFFI() const;
  inline static FixedDecimalRoundingIncrement FromFFI(diplomat::capi::FixedDecimalRoundingIncrement c_enum);
private:
    Value value;
};


#endif // FixedDecimalRoundingIncrement_D_HPP
