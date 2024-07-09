#ifndef ICU4XLineBreakStrictness_D_HPP
#define ICU4XLineBreakStrictness_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XLineBreakStrictness {
      ICU4XLineBreakStrictness_Loose = 0,
      ICU4XLineBreakStrictness_Normal = 1,
      ICU4XLineBreakStrictness_Strict = 2,
      ICU4XLineBreakStrictness_Anywhere = 3,
    } ICU4XLineBreakStrictness;
}

class ICU4XLineBreakStrictness {
public:
  enum Value {
    Loose = 0,
    Normal = 1,
    Strict = 2,
    Anywhere = 3,
  };

  ICU4XLineBreakStrictness() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XLineBreakStrictness(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XLineBreakStrictness AsFFI() const;
  inline static ICU4XLineBreakStrictness FromFFI(capi::ICU4XLineBreakStrictness c_enum);
private:
    Value value;
};


#endif // ICU4XLineBreakStrictness_D_HPP
