#ifndef LanguageDisplay_D_HPP
#define LanguageDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum LanguageDisplay {
      LanguageDisplay_Dialect = 0,
      LanguageDisplay_Standard = 1,
    } LanguageDisplay;
} // namespace capi
} // namespace

class LanguageDisplay {
public:
  enum Value {
    Dialect = 0,
    Standard = 1,
  };

  LanguageDisplay() = default;
  // Implicit conversions between enum and ::Value
  constexpr LanguageDisplay(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::LanguageDisplay AsFFI() const;
  inline static LanguageDisplay FromFFI(diplomat::capi::LanguageDisplay c_enum);
private:
    Value value;
};


#endif // LanguageDisplay_D_HPP
