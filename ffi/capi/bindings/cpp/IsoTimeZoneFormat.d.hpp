#ifndef IsoTimeZoneFormat_D_HPP
#define IsoTimeZoneFormat_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    typedef enum IsoTimeZoneFormat {
      IsoTimeZoneFormat_Basic = 0,
      IsoTimeZoneFormat_Extended = 1,
      IsoTimeZoneFormat_UtcBasic = 2,
      IsoTimeZoneFormat_UtcExtended = 3,
    } IsoTimeZoneFormat;
} // namespace capi
} // namespace

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

  inline diplomat::capi::IsoTimeZoneFormat AsFFI() const;
  inline static IsoTimeZoneFormat FromFFI(diplomat::capi::IsoTimeZoneFormat c_enum);
private:
    Value value;
};


#endif // IsoTimeZoneFormat_D_HPP
