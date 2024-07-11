#ifndef LocaleDirection_D_HPP
#define LocaleDirection_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum LocaleDirection {
      LocaleDirection_LeftToRight = 0,
      LocaleDirection_RightToLeft = 1,
      LocaleDirection_Unknown = 2,
    } LocaleDirection;
}

class LocaleDirection {
public:
  enum Value {
    LeftToRight = 0,
    RightToLeft = 1,
    Unknown = 2,
  };

  LocaleDirection() = default;
  // Implicit conversions between enum and ::Value
  constexpr LocaleDirection(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::LocaleDirection AsFFI() const;
  inline static LocaleDirection FromFFI(capi::LocaleDirection c_enum);
private:
    Value value;
};


#endif // LocaleDirection_D_HPP
