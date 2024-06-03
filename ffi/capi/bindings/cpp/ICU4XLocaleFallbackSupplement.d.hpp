#ifndef ICU4XLocaleFallbackSupplement_D_HPP
#define ICU4XLocaleFallbackSupplement_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocaleFallbackSupplement.d.h"


class ICU4XLocaleFallbackSupplement {
public:
  enum Value {
    None = 0,
    Collation = 1,
  };

  ICU4XLocaleFallbackSupplement() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XLocaleFallbackSupplement(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XLocaleFallbackSupplement AsFFI() const;
  inline static ICU4XLocaleFallbackSupplement FromFFI(capi::ICU4XLocaleFallbackSupplement c_enum);
private:
    Value value;
};


#endif // ICU4XLocaleFallbackSupplement_D_HPP
