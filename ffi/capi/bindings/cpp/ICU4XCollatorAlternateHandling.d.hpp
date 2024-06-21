#ifndef ICU4XCollatorAlternateHandling_D_HPP
#define ICU4XCollatorAlternateHandling_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XCollatorAlternateHandling {
      ICU4XCollatorAlternateHandling_Auto = 0,
      ICU4XCollatorAlternateHandling_NonIgnorable = 1,
      ICU4XCollatorAlternateHandling_Shifted = 2,
    } ICU4XCollatorAlternateHandling;
}

class ICU4XCollatorAlternateHandling {
public:
  enum Value {
    Auto = 0,
    NonIgnorable = 1,
    Shifted = 2,
  };

  ICU4XCollatorAlternateHandling() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XCollatorAlternateHandling(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XCollatorAlternateHandling AsFFI() const;
  inline static ICU4XCollatorAlternateHandling FromFFI(capi::ICU4XCollatorAlternateHandling c_enum);
private:
    Value value;
};


#endif // ICU4XCollatorAlternateHandling_D_HPP
