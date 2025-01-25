#ifndef icu4x_ZoneStyle_D_HPP
#define icu4x_ZoneStyle_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum ZoneStyle {
      ZoneStyle_Z = 0,
      ZoneStyle_Zs = 1,
      ZoneStyle_O = 2,
      ZoneStyle_Os = 3,
      ZoneStyle_V = 4,
      ZoneStyle_Vs = 5,
      ZoneStyle_L = 6,
    };
    
    typedef struct ZoneStyle_option {union { ZoneStyle ok; }; bool is_ok; } ZoneStyle_option;
} // namespace capi
} // namespace

namespace icu4x {
class ZoneStyle {
public:
  enum Value {
    Z = 0,
    Zs = 1,
    O = 2,
    Os = 3,
    V = 4,
    Vs = 5,
    L = 6,
  };

  ZoneStyle() = default;
  // Implicit conversions between enum and ::Value
  constexpr ZoneStyle(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::ZoneStyle AsFFI() const;
  inline static icu4x::ZoneStyle FromFFI(icu4x::capi::ZoneStyle c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_ZoneStyle_D_HPP
