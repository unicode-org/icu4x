#ifndef IsoTimeZoneMinuteDisplay_D_HPP
#define IsoTimeZoneMinuteDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum IsoTimeZoneMinuteDisplay {
      IsoTimeZoneMinuteDisplay_Required = 0,
      IsoTimeZoneMinuteDisplay_Optional = 1,
    } IsoTimeZoneMinuteDisplay;
} // namespace capi
} // namespace

class IsoTimeZoneMinuteDisplay {
public:
  enum Value {
    Required = 0,
    Optional = 1,
  };

  IsoTimeZoneMinuteDisplay() = default;
  // Implicit conversions between enum and ::Value
  constexpr IsoTimeZoneMinuteDisplay(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline diplomat::capi::IsoTimeZoneMinuteDisplay AsFFI() const;
  inline static IsoTimeZoneMinuteDisplay FromFFI(diplomat::capi::IsoTimeZoneMinuteDisplay c_enum);
private:
    Value value;
};


#endif // IsoTimeZoneMinuteDisplay_D_HPP
