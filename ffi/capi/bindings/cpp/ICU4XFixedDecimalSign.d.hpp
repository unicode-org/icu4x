#ifndef ICU4XFixedDecimalSign_D_HPP
#define ICU4XFixedDecimalSign_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XFixedDecimalSign {
      ICU4XFixedDecimalSign_None = 0,
      ICU4XFixedDecimalSign_Negative = 1,
      ICU4XFixedDecimalSign_Positive = 2,
    } ICU4XFixedDecimalSign;
}

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
