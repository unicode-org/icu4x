#ifndef ICU4XListLength_D_HPP
#define ICU4XListLength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XListLength {
      ICU4XListLength_Wide = 0,
      ICU4XListLength_Short = 1,
      ICU4XListLength_Narrow = 2,
    } ICU4XListLength;
}

class ICU4XListLength {
public:
  enum Value {
    Wide = 0,
    Short = 1,
    Narrow = 2,
  };

  ICU4XListLength() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XListLength(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XListLength AsFFI() const;
  inline static ICU4XListLength FromFFI(capi::ICU4XListLength c_enum);
private:
    Value value;
};


#endif // ICU4XListLength_D_HPP
