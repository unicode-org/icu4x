#ifndef icu4x_IsoTimeZoneOptions_D_HPP
#define icu4x_IsoTimeZoneOptions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "../diplomat_runtime.hpp"
#include "IsoTimeZoneFormat.d.hpp"
#include "IsoTimeZoneMinuteDisplay.d.hpp"
#include "IsoTimeZoneSecondDisplay.d.hpp"

namespace icu4x {
class IsoTimeZoneFormat;
class IsoTimeZoneMinuteDisplay;
class IsoTimeZoneSecondDisplay;
}


namespace icu4x {
namespace capi {
    struct IsoTimeZoneOptions {
      icu4x::capi::IsoTimeZoneFormat format;
      icu4x::capi::IsoTimeZoneMinuteDisplay minutes;
      icu4x::capi::IsoTimeZoneSecondDisplay seconds;
    };
} // namespace capi
} // namespace


namespace icu4x {
struct IsoTimeZoneOptions {
  icu4x::IsoTimeZoneFormat format;
  icu4x::IsoTimeZoneMinuteDisplay minutes;
  icu4x::IsoTimeZoneSecondDisplay seconds;

  inline icu4x::capi::IsoTimeZoneOptions AsFFI() const;
  inline static icu4x::IsoTimeZoneOptions FromFFI(icu4x::capi::IsoTimeZoneOptions c_struct);
};

} // namespace
#endif // icu4x_IsoTimeZoneOptions_D_HPP
