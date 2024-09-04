#ifndef icu4x_CollatorNumeric_D_HPP
#define icu4x_CollatorNumeric_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum CollatorNumeric {
      CollatorNumeric_Auto = 0,
      CollatorNumeric_Off = 1,
      CollatorNumeric_On = 2,
    };
    
    typedef struct CollatorNumeric_option {union { CollatorNumeric ok; }; bool is_ok; } CollatorNumeric_option;
} // namespace capi
} // namespace

namespace icu4x {
class CollatorNumeric {
public:
  enum Value {
    Auto = 0,
    Off = 1,
    On = 2,
  };

  CollatorNumeric() = default;
  // Implicit conversions between enum and ::Value
  constexpr CollatorNumeric(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::CollatorNumeric AsFFI() const;
  inline static icu4x::CollatorNumeric FromFFI(icu4x::capi::CollatorNumeric c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_CollatorNumeric_D_HPP
