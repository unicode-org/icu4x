#ifndef ICU4XTimeLength_D_HPP
#define ICU4XTimeLength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XTimeLength.d.h"


class ICU4XTimeLength {
public:
  enum Value {
    Full = 0,
    Long = 1,
    Medium = 2,
    Short = 3,
  };

  ICU4XTimeLength() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XTimeLength(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XTimeLength AsFFI() const;
  inline static ICU4XTimeLength FromFFI(capi::ICU4XTimeLength c_enum);
private:
    Value value;
};


#endif // ICU4XTimeLength_D_HPP
