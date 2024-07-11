#ifndef FixedDecimalSign_D_HPP
#define FixedDecimalSign_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum FixedDecimalSign {
      FixedDecimalSign_None = 0,
      FixedDecimalSign_Negative = 1,
      FixedDecimalSign_Positive = 2,
    } FixedDecimalSign;
}

class FixedDecimalSign {
public:
  enum Value {
    None = 0,
    Negative = 1,
    Positive = 2,
  };

  FixedDecimalSign() = default;
  // Implicit conversions between enum and ::Value
  constexpr FixedDecimalSign(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::FixedDecimalSign AsFFI() const;
  inline static FixedDecimalSign FromFFI(capi::FixedDecimalSign c_enum);
private:
    Value value;
};


#endif // FixedDecimalSign_D_HPP
