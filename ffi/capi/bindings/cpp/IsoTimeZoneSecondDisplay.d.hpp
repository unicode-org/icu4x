#ifndef IsoTimeZoneSecondDisplay_D_HPP
#define IsoTimeZoneSecondDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum IsoTimeZoneSecondDisplay {
      IsoTimeZoneSecondDisplay_Optional = 0,
      IsoTimeZoneSecondDisplay_Never = 1,
    } IsoTimeZoneSecondDisplay;
}

class IsoTimeZoneSecondDisplay {
public:
  enum Value {
    Optional = 0,
    Never = 1,
  };

  IsoTimeZoneSecondDisplay() = default;
  // Implicit conversions between enum and ::Value
  constexpr IsoTimeZoneSecondDisplay(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::IsoTimeZoneSecondDisplay AsFFI() const;
  inline static IsoTimeZoneSecondDisplay FromFFI(capi::IsoTimeZoneSecondDisplay c_enum);
private:
    Value value;
};


#endif // IsoTimeZoneSecondDisplay_D_HPP
