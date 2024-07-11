#ifndef PluralCategory_D_HPP
#define PluralCategory_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum PluralCategory {
      PluralCategory_Zero = 0,
      PluralCategory_One = 1,
      PluralCategory_Two = 2,
      PluralCategory_Few = 3,
      PluralCategory_Many = 4,
      PluralCategory_Other = 5,
    } PluralCategory;
}

class PluralCategory {
public:
  enum Value {
    Zero = 0,
    One = 1,
    Two = 2,
    Few = 3,
    Many = 4,
    Other = 5,
  };

  PluralCategory() = default;
  // Implicit conversions between enum and ::Value
  constexpr PluralCategory(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline static std::optional<PluralCategory> get_for_cldr_string(std::string_view s);

  inline capi::PluralCategory AsFFI() const;
  inline static PluralCategory FromFFI(capi::PluralCategory c_enum);
private:
    Value value;
};


#endif // PluralCategory_D_HPP
