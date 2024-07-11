#ifndef FixedDecimalSignDisplay_D_HPP
#define FixedDecimalSignDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum FixedDecimalSignDisplay {
      FixedDecimalSignDisplay_Auto = 0,
      FixedDecimalSignDisplay_Never = 1,
      FixedDecimalSignDisplay_Always = 2,
      FixedDecimalSignDisplay_ExceptZero = 3,
      FixedDecimalSignDisplay_Negative = 4,
    } FixedDecimalSignDisplay;
}

class FixedDecimalSignDisplay {
public:
  enum Value {
    Auto = 0,
    Never = 1,
    Always = 2,
    ExceptZero = 3,
    Negative = 4,
  };

  FixedDecimalSignDisplay() = default;
  // Implicit conversions between enum and ::Value
  constexpr FixedDecimalSignDisplay(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::FixedDecimalSignDisplay AsFFI() const;
  inline static FixedDecimalSignDisplay FromFFI(capi::FixedDecimalSignDisplay c_enum);
private:
    Value value;
};


#endif // FixedDecimalSignDisplay_D_HPP
