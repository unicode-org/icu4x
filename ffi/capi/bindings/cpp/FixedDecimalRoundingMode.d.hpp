#ifndef FixedDecimalRoundingMode_D_HPP
#define FixedDecimalRoundingMode_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum FixedDecimalRoundingMode {
      FixedDecimalRoundingMode_Ceil = 0,
      FixedDecimalRoundingMode_Expand = 1,
      FixedDecimalRoundingMode_Floor = 2,
      FixedDecimalRoundingMode_Trunc = 3,
      FixedDecimalRoundingMode_HalfCeil = 4,
      FixedDecimalRoundingMode_HalfExpand = 5,
      FixedDecimalRoundingMode_HalfFloor = 6,
      FixedDecimalRoundingMode_HalfTrunc = 7,
      FixedDecimalRoundingMode_HalfEven = 8,
    };
} // namespace capi
} // namespace

class FixedDecimalRoundingMode {
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

  FixedDecimalRoundingMode() = default;
  // Implicit conversions between enum and ::Value
  constexpr FixedDecimalRoundingMode(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::FixedDecimalRoundingMode AsFFI() const;
  inline static FixedDecimalRoundingMode FromFFI(diplomat::capi::FixedDecimalRoundingMode c_enum);
private:
    Value value;
};


#endif // FixedDecimalRoundingMode_D_HPP
