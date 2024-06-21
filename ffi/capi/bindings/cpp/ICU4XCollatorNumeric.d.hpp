#ifndef ICU4XCollatorNumeric_D_HPP
#define ICU4XCollatorNumeric_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XCollatorNumeric {
      ICU4XCollatorNumeric_Auto = 0,
      ICU4XCollatorNumeric_Off = 1,
      ICU4XCollatorNumeric_On = 2,
    } ICU4XCollatorNumeric;
}

class ICU4XCollatorNumeric {
public:
  enum Value {
    Auto = 0,
    Off = 1,
    On = 2,
  };

  ICU4XCollatorNumeric() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XCollatorNumeric(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XCollatorNumeric AsFFI() const;
  inline static ICU4XCollatorNumeric FromFFI(capi::ICU4XCollatorNumeric c_enum);
private:
    Value value;
};


#endif // ICU4XCollatorNumeric_D_HPP
