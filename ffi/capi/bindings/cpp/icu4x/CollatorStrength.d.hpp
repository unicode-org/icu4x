#ifndef icu4x_CollatorStrength_D_HPP
#define icu4x_CollatorStrength_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum CollatorStrength {
      CollatorStrength_Auto = 0,
      CollatorStrength_Primary = 1,
      CollatorStrength_Secondary = 2,
      CollatorStrength_Tertiary = 3,
      CollatorStrength_Quaternary = 4,
      CollatorStrength_Identical = 5,
    };
} // namespace capi
} // namespace

namespace icu4x {
class CollatorStrength {
public:
  enum Value {
    Auto = 0,
    Primary = 1,
    Secondary = 2,
    Tertiary = 3,
    Quaternary = 4,
    Identical = 5,
  };

  CollatorStrength() = default;
  // Implicit conversions between enum and ::Value
  constexpr CollatorStrength(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::CollatorStrength AsFFI() const;
  inline static icu4x::CollatorStrength FromFFI(icu4x::capi::CollatorStrength c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_CollatorStrength_D_HPP
