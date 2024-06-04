#ifndef ICU4XCollatorBackwardSecondLevel_D_HPP
#define ICU4XCollatorBackwardSecondLevel_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollatorBackwardSecondLevel.d.h"


class ICU4XCollatorBackwardSecondLevel {
public:
  enum Value {
    Auto = 0,
    Off = 1,
    On = 2,
  };

  ICU4XCollatorBackwardSecondLevel() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XCollatorBackwardSecondLevel(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XCollatorBackwardSecondLevel AsFFI() const;
  inline static ICU4XCollatorBackwardSecondLevel FromFFI(capi::ICU4XCollatorBackwardSecondLevel c_enum);
private:
    Value value;
};


#endif // ICU4XCollatorBackwardSecondLevel_D_HPP
