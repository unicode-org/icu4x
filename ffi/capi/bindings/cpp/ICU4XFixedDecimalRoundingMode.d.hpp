#ifndef ICU4XFixedDecimalRoundingMode_D_HPP
#define ICU4XFixedDecimalRoundingMode_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XFixedDecimalRoundingMode {
      ICU4XFixedDecimalRoundingMode_Ceil = 0,
      ICU4XFixedDecimalRoundingMode_Expand = 1,
      ICU4XFixedDecimalRoundingMode_Floor = 2,
      ICU4XFixedDecimalRoundingMode_Trunc = 3,
      ICU4XFixedDecimalRoundingMode_HalfCeil = 4,
      ICU4XFixedDecimalRoundingMode_HalfExpand = 5,
      ICU4XFixedDecimalRoundingMode_HalfFloor = 6,
      ICU4XFixedDecimalRoundingMode_HalfTrunc = 7,
      ICU4XFixedDecimalRoundingMode_HalfEven = 8,
    } ICU4XFixedDecimalRoundingMode;
}

class ICU4XFixedDecimalRoundingMode {
public:
  enum Value {
    Ceil = 0,
    Expand = 1,
    Floor = 2,
    Trunc = 3,
    HalfCeil = 4,
    HalfExpand = 5,
    HalfFloor = 6,
    HalfTrunc = 7,
    HalfEven = 8,
  };

  ICU4XFixedDecimalRoundingMode() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XFixedDecimalRoundingMode(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XFixedDecimalRoundingMode AsFFI() const;
  inline static ICU4XFixedDecimalRoundingMode FromFFI(capi::ICU4XFixedDecimalRoundingMode c_enum);
private:
    Value value;
};


#endif // ICU4XFixedDecimalRoundingMode_D_HPP
