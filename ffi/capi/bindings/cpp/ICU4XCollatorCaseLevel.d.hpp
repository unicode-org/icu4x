#ifndef ICU4XCollatorCaseLevel_D_HPP
#define ICU4XCollatorCaseLevel_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XCollatorCaseLevel {
      ICU4XCollatorCaseLevel_Auto = 0,
      ICU4XCollatorCaseLevel_Off = 1,
      ICU4XCollatorCaseLevel_On = 2,
    } ICU4XCollatorCaseLevel;
}

class ICU4XCollatorCaseLevel {
public:
  enum Value {
    Auto = 0,
    Off = 1,
    On = 2,
  };

  ICU4XCollatorCaseLevel() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XCollatorCaseLevel(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XCollatorCaseLevel AsFFI() const;
  inline static ICU4XCollatorCaseLevel FromFFI(capi::ICU4XCollatorCaseLevel c_enum);
private:
    Value value;
};


#endif // ICU4XCollatorCaseLevel_D_HPP
