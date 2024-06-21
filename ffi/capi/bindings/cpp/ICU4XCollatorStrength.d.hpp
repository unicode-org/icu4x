#ifndef ICU4XCollatorStrength_D_HPP
#define ICU4XCollatorStrength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XCollatorStrength {
      ICU4XCollatorStrength_Auto = 0,
      ICU4XCollatorStrength_Primary = 1,
      ICU4XCollatorStrength_Secondary = 2,
      ICU4XCollatorStrength_Tertiary = 3,
      ICU4XCollatorStrength_Quaternary = 4,
      ICU4XCollatorStrength_Identical = 5,
    } ICU4XCollatorStrength;
}

class ICU4XCollatorStrength {
public:
  enum Value {
    Auto = 0,
    Primary = 1,
    Secondary = 2,
    Tertiary = 3,
    Quaternary = 4,
    Identical = 5,
  };

  ICU4XCollatorStrength() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XCollatorStrength(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XCollatorStrength AsFFI() const;
  inline static ICU4XCollatorStrength FromFFI(capi::ICU4XCollatorStrength c_enum);
private:
    Value value;
};


#endif // ICU4XCollatorStrength_D_HPP
