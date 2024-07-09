#ifndef ICU4XLocaleDirection_D_HPP
#define ICU4XLocaleDirection_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XLocaleDirection {
      ICU4XLocaleDirection_LeftToRight = 0,
      ICU4XLocaleDirection_RightToLeft = 1,
      ICU4XLocaleDirection_Unknown = 2,
    } ICU4XLocaleDirection;
}

class ICU4XLocaleDirection {
public:
  enum Value {
    LeftToRight = 0,
    RightToLeft = 1,
    Unknown = 2,
  };

  ICU4XLocaleDirection() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XLocaleDirection(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XLocaleDirection AsFFI() const;
  inline static ICU4XLocaleDirection FromFFI(capi::ICU4XLocaleDirection c_enum);
private:
    Value value;
};


#endif // ICU4XLocaleDirection_D_HPP
