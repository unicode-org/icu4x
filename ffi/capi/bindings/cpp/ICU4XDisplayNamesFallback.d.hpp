#ifndef ICU4XDisplayNamesFallback_D_HPP
#define ICU4XDisplayNamesFallback_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XDisplayNamesFallback.d.h"


class ICU4XDisplayNamesFallback {
public:
  enum Value {
    Code = 0,
    None = 1,
  };

  ICU4XDisplayNamesFallback() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XDisplayNamesFallback(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XDisplayNamesFallback AsFFI() const;
  inline static ICU4XDisplayNamesFallback FromFFI(capi::ICU4XDisplayNamesFallback c_enum);
private:
    Value value;
};


#endif // ICU4XDisplayNamesFallback_D_HPP
