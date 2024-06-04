#ifndef ICU4XIsoTimeZoneFormat_D_HPP
#define ICU4XIsoTimeZoneFormat_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XIsoTimeZoneFormat.d.h"


class ICU4XIsoTimeZoneFormat {
public:
  enum Value {
    Basic = 0,
    Extended = 1,
    UtcBasic = 2,
    UtcExtended = 3,
  };

  ICU4XIsoTimeZoneFormat() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XIsoTimeZoneFormat(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XIsoTimeZoneFormat AsFFI() const;
  inline static ICU4XIsoTimeZoneFormat FromFFI(capi::ICU4XIsoTimeZoneFormat c_enum);
private:
    Value value;
};


#endif // ICU4XIsoTimeZoneFormat_D_HPP
