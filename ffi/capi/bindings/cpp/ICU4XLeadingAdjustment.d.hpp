#ifndef ICU4XLeadingAdjustment_D_HPP
#define ICU4XLeadingAdjustment_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XLeadingAdjustment {
      ICU4XLeadingAdjustment_Auto = 0,
      ICU4XLeadingAdjustment_None = 1,
      ICU4XLeadingAdjustment_ToCased = 2,
    } ICU4XLeadingAdjustment;
}

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
