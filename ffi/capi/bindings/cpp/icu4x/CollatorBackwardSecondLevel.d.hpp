#ifndef icu4x_CollatorBackwardSecondLevel_D_HPP
#define icu4x_CollatorBackwardSecondLevel_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum CollatorBackwardSecondLevel {
      CollatorBackwardSecondLevel_Auto = 0,
      CollatorBackwardSecondLevel_Off = 1,
      CollatorBackwardSecondLevel_On = 2,
    };
} // namespace capi
} // namespace

namespace icu4x {
class CollatorBackwardSecondLevel {
public:
  enum Value {
    Auto = 0,
    Off = 1,
    On = 2,
  };

  CollatorBackwardSecondLevel() = default;
  // Implicit conversions between enum and ::Value
  constexpr CollatorBackwardSecondLevel(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::CollatorBackwardSecondLevel AsFFI() const;
  inline static icu4x::CollatorBackwardSecondLevel FromFFI(icu4x::capi::CollatorBackwardSecondLevel c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_CollatorBackwardSecondLevel_D_HPP
