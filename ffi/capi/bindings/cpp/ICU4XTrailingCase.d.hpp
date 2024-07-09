#ifndef ICU4XTrailingCase_D_HPP
#define ICU4XTrailingCase_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XTrailingCase {
      ICU4XTrailingCase_Lower = 0,
      ICU4XTrailingCase_Unchanged = 1,
    } ICU4XTrailingCase;
}

class ICU4XTrailingCase {
public:
  enum Value {
    Lower = 0,
    Unchanged = 1,
  };

  ICU4XTrailingCase() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XTrailingCase(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XTrailingCase AsFFI() const;
  inline static ICU4XTrailingCase FromFFI(capi::ICU4XTrailingCase c_enum);
private:
    Value value;
};


#endif // ICU4XTrailingCase_D_HPP
