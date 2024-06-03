#ifndef ICU4XLineBreakWordOption_D_HPP
#define ICU4XLineBreakWordOption_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLineBreakWordOption.d.h"


class ICU4XLineBreakWordOption {
public:
  enum Value {
    Normal = 0,
    BreakAll = 1,
    KeepAll = 2,
  };

  ICU4XLineBreakWordOption() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XLineBreakWordOption(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XLineBreakWordOption AsFFI() const;
  inline static ICU4XLineBreakWordOption FromFFI(capi::ICU4XLineBreakWordOption c_enum);
private:
    Value value;
};


#endif // ICU4XLineBreakWordOption_D_HPP
