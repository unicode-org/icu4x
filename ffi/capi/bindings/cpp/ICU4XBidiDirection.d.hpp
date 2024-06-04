#ifndef ICU4XBidiDirection_D_HPP
#define ICU4XBidiDirection_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XBidiDirection.d.h"


class ICU4XBidiDirection {
public:
  enum Value {
    Ltr = 0,
    Rtl = 1,
    Mixed = 2,
  };

  ICU4XBidiDirection() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XBidiDirection(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XBidiDirection AsFFI() const;
  inline static ICU4XBidiDirection FromFFI(capi::ICU4XBidiDirection c_enum);
private:
    Value value;
};


#endif // ICU4XBidiDirection_D_HPP
