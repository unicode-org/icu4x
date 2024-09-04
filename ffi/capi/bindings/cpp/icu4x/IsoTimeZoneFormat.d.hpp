#ifndef icu4x_IsoTimeZoneFormat_D_HPP
#define icu4x_IsoTimeZoneFormat_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"


namespace icu4x {
namespace capi {
    enum IsoTimeZoneFormat {
      IsoTimeZoneFormat_Basic = 0,
      IsoTimeZoneFormat_Extended = 1,
      IsoTimeZoneFormat_UtcBasic = 2,
      IsoTimeZoneFormat_UtcExtended = 3,
    };
    
    typedef struct IsoTimeZoneFormat_option {union { IsoTimeZoneFormat ok; }; bool is_ok; } IsoTimeZoneFormat_option;
} // namespace capi
} // namespace

namespace icu4x {
class IsoTimeZoneFormat {
public:
  enum Value {
    Basic = 0,
    Extended = 1,
    UtcBasic = 2,
    UtcExtended = 3,
  };

  IsoTimeZoneFormat() = default;
  // Implicit conversions between enum and ::Value
  constexpr IsoTimeZoneFormat(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline icu4x::capi::IsoTimeZoneFormat AsFFI() const;
  inline static icu4x::IsoTimeZoneFormat FromFFI(icu4x::capi::IsoTimeZoneFormat c_enum);
private:
    Value value;
};

} // namespace
#endif // icu4x_IsoTimeZoneFormat_D_HPP
