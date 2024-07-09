#ifndef ICU4XIsoTimeZoneMinuteDisplay_D_HPP
#define ICU4XIsoTimeZoneMinuteDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XIsoTimeZoneMinuteDisplay {
      ICU4XIsoTimeZoneMinuteDisplay_Required = 0,
      ICU4XIsoTimeZoneMinuteDisplay_Optional = 1,
    } ICU4XIsoTimeZoneMinuteDisplay;
}

class ICU4XIsoTimeZoneMinuteDisplay {
public:
  enum Value {
    Required = 0,
    Optional = 1,
  };

  ICU4XIsoTimeZoneMinuteDisplay() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XIsoTimeZoneMinuteDisplay(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XIsoTimeZoneMinuteDisplay AsFFI() const;
  inline static ICU4XIsoTimeZoneMinuteDisplay FromFFI(capi::ICU4XIsoTimeZoneMinuteDisplay c_enum);
private:
    Value value;
};


#endif // ICU4XIsoTimeZoneMinuteDisplay_D_HPP
