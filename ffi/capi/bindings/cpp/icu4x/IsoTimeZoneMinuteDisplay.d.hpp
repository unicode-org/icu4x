#ifndef icu4x_IsoTimeZoneMinuteDisplay_D_HPP
#define icu4x_IsoTimeZoneMinuteDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum IsoTimeZoneMinuteDisplay {
      IsoTimeZoneMinuteDisplay_Required = 0,
      IsoTimeZoneMinuteDisplay_Optional = 1,
    };
} // namespace capi
} // namespace

namespace icu4x {
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

  inline icu4x::capi::IsoTimeZoneMinuteDisplay AsFFI() const;
  inline static icu4x::IsoTimeZoneMinuteDisplay FromFFI(icu4x::capi::IsoTimeZoneMinuteDisplay c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_IsoTimeZoneMinuteDisplay_D_HPP
