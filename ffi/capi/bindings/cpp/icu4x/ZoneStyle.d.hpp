#ifndef icu4x_ZoneStyle_D_HPP
#define icu4x_ZoneStyle_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum ZoneStyle {
      ZoneStyle_SpecificLong = 0,
      ZoneStyle_SpecificShort = 1,
      ZoneStyle_LocalizedOffsetLong = 2,
      ZoneStyle_LocalizedOffsetShort = 3,
      ZoneStyle_GenericLong = 4,
      ZoneStyle_GenericShort = 5,
      ZoneStyle_Location = 6,
      ZoneStyle_ExemplarCity = 7,
    };
    
    typedef struct ZoneStyle_option {union { ZoneStyle ok; }; bool is_ok; } ZoneStyle_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * See the [Rust documentation for `ZoneStyle`](https://docs.rs/icu_datetime/latest/icu_datetime/fieldsets/builder/enum.ZoneStyle.html) for more information.
 */
class ZoneStyle {
public:
  enum Value {
    SpecificLong = 0,
    SpecificShort = 1,
    LocalizedOffsetLong = 2,
    LocalizedOffsetShort = 3,
    GenericLong = 4,
    GenericShort = 5,
    Location = 6,
    ExemplarCity = 7,
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
