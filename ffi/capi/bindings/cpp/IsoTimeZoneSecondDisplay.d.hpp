#ifndef IsoTimeZoneSecondDisplay_D_HPP
#define IsoTimeZoneSecondDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    enum IsoTimeZoneSecondDisplay {
      IsoTimeZoneSecondDisplay_Optional = 0,
      IsoTimeZoneSecondDisplay_Never = 1,
    };
} // namespace capi
} // namespace

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

  inline diplomat::capi::IsoTimeZoneSecondDisplay AsFFI() const;
  inline static IsoTimeZoneSecondDisplay FromFFI(diplomat::capi::IsoTimeZoneSecondDisplay c_enum);
private:
    Value value;
};


#endif // IsoTimeZoneSecondDisplay_D_HPP
