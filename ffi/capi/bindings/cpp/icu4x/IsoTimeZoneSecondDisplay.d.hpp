#ifndef icu4x_IsoTimeZoneSecondDisplay_D_HPP
#define icu4x_IsoTimeZoneSecondDisplay_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum IsoTimeZoneSecondDisplay {
      IsoTimeZoneSecondDisplay_Optional = 0,
      IsoTimeZoneSecondDisplay_Never = 1,
    };
    
    typedef struct IsoTimeZoneSecondDisplay_option {union { IsoTimeZoneSecondDisplay ok; }; bool is_ok; } IsoTimeZoneSecondDisplay_option;
} // namespace capi
} // namespace

namespace icu4x {
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

  inline icu4x::capi::IsoTimeZoneSecondDisplay AsFFI() const;
  inline static icu4x::IsoTimeZoneSecondDisplay FromFFI(icu4x::capi::IsoTimeZoneSecondDisplay c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_IsoTimeZoneSecondDisplay_D_HPP
