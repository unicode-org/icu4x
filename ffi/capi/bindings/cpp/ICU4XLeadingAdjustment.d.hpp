#ifndef ICU4XLeadingAdjustment_D_HPP
#define ICU4XLeadingAdjustment_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLeadingAdjustment.d.h"


class ICU4XLeadingAdjustment {
public:
  enum Value {
    Auto = 0,
    None = 1,
    ToCased = 2,
  };

  ICU4XLeadingAdjustment() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XLeadingAdjustment(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XLeadingAdjustment AsFFI() const;
  inline static ICU4XLeadingAdjustment FromFFI(capi::ICU4XLeadingAdjustment c_enum);
private:
    Value value;
};


#endif // ICU4XLeadingAdjustment_D_HPP
