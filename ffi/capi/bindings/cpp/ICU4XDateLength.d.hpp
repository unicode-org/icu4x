#ifndef ICU4XDateLength_D_HPP
#define ICU4XDateLength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XDateLength {
      ICU4XDateLength_Full = 0,
      ICU4XDateLength_Long = 1,
      ICU4XDateLength_Medium = 2,
      ICU4XDateLength_Short = 3,
    } ICU4XDateLength;
}

class ICU4XDateLength {
public:
  enum Value {
    Full = 0,
    Long = 1,
    Medium = 2,
    Short = 3,
  };

  ICU4XDateLength() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XDateLength(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XDateLength AsFFI() const;
  inline static ICU4XDateLength FromFFI(capi::ICU4XDateLength c_enum);
private:
    Value value;
};


#endif // ICU4XDateLength_D_HPP
