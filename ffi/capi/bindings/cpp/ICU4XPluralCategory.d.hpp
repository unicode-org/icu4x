#ifndef ICU4XPluralCategory_D_HPP
#define ICU4XPluralCategory_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XPluralCategory.d.h"


class ICU4XPluralCategory {
public:
  enum Value {
    Zero = 0,
    One = 1,
    Two = 2,
    Few = 3,
    Many = 4,
    Other = 5,
  };

  ICU4XPluralCategory() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XPluralCategory(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline static std::optional<ICU4XPluralCategory> get_for_cldr_string(std::string_view s);

  inline capi::ICU4XPluralCategory AsFFI() const;
  inline static ICU4XPluralCategory FromFFI(capi::ICU4XPluralCategory c_enum);
private:
    Value value;
};


#endif // ICU4XPluralCategory_D_HPP
