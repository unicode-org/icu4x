#ifndef CollatorNumeric_D_HPP
#define CollatorNumeric_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum CollatorNumeric {
      CollatorNumeric_Auto = 0,
      CollatorNumeric_Off = 1,
      CollatorNumeric_On = 2,
    };
} // namespace capi
} // namespace

class CollatorNumeric {
public:
  enum Value {
    Auto = 0,
    Off = 1,
    On = 2,
  };

  CollatorNumeric() = default;
  // Implicit conversions between enum and ::Value
  constexpr CollatorNumeric(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::CollatorNumeric AsFFI() const;
  inline static CollatorNumeric FromFFI(diplomat::capi::CollatorNumeric c_enum);
private:
    Value value;
};


#endif // CollatorNumeric_D_HPP
