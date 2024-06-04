#ifndef ICU4XDisplayNamesStyle_D_HPP
#define ICU4XDisplayNamesStyle_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDisplayNamesStyle.d.h"


class ICU4XDisplayNamesStyle {
public:
  enum Value {
    Auto = 0,
    Narrow = 1,
    Short = 2,
    Long = 3,
    Menu = 4,
  };

  ICU4XDisplayNamesStyle() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XDisplayNamesStyle(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XDisplayNamesStyle AsFFI() const;
  inline static ICU4XDisplayNamesStyle FromFFI(capi::ICU4XDisplayNamesStyle c_enum);
private:
    Value value;
};


#endif // ICU4XDisplayNamesStyle_D_HPP
