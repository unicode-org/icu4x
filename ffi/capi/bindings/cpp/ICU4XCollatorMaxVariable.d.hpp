#ifndef ICU4XCollatorMaxVariable_D_HPP
#define ICU4XCollatorMaxVariable_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollatorMaxVariable.d.h"


class ICU4XCollatorMaxVariable {
public:
  enum Value {
    Auto = 0,
    Space = 1,
    Punctuation = 2,
    Symbol = 3,
    Currency = 4,
  };

  ICU4XCollatorMaxVariable() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XCollatorMaxVariable(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XCollatorMaxVariable AsFFI() const;
  inline static ICU4XCollatorMaxVariable FromFFI(capi::ICU4XCollatorMaxVariable c_enum);
private:
    Value value;
};


#endif // ICU4XCollatorMaxVariable_D_HPP
