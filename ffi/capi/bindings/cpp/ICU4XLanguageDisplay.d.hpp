#ifndef ICU4XLanguageDisplay_D_HPP
#define ICU4XLanguageDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XLanguageDisplay {
      ICU4XLanguageDisplay_Dialect = 0,
      ICU4XLanguageDisplay_Standard = 1,
    } ICU4XLanguageDisplay;
}

class ICU4XLanguageDisplay {
public:
  enum Value {
    Dialect = 0,
    Standard = 1,
  };

  ICU4XLanguageDisplay() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XLanguageDisplay(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XLanguageDisplay AsFFI() const;
  inline static ICU4XLanguageDisplay FromFFI(capi::ICU4XLanguageDisplay c_enum);
private:
    Value value;
};


#endif // ICU4XLanguageDisplay_D_HPP
