#ifndef ICU4XCollatorCaseFirst_D_HPP
#define ICU4XCollatorCaseFirst_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XCollatorCaseFirst.d.h"


class ICU4XCollatorCaseFirst {
public:
  enum Value {
    Auto = 0,
    Off = 1,
    LowerFirst = 2,
    UpperFirst = 3,
  };

  ICU4XCollatorCaseFirst() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XCollatorCaseFirst(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XCollatorCaseFirst AsFFI() const;
  inline static ICU4XCollatorCaseFirst FromFFI(capi::ICU4XCollatorCaseFirst c_enum);
private:
    Value value;
};


#endif // ICU4XCollatorCaseFirst_D_HPP
