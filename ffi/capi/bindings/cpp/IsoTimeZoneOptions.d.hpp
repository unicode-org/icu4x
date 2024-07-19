#ifndef IsoTimeZoneOptions_D_HPP
#define IsoTimeZoneOptions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "IsoTimeZoneFormat.d.hpp"
#include "IsoTimeZoneMinuteDisplay.d.hpp"
#include "IsoTimeZoneSecondDisplay.d.hpp"
#include "diplomat_runtime.hpp"

class IsoTimeZoneFormat;
class IsoTimeZoneMinuteDisplay;
class IsoTimeZoneSecondDisplay;


namespace diplomat {
namespace capi {
    struct IsoTimeZoneOptions {
      diplomat::capi::IsoTimeZoneFormat format;
      diplomat::capi::IsoTimeZoneMinuteDisplay minutes;
      diplomat::capi::IsoTimeZoneSecondDisplay seconds;
    };
} // namespace capi
} // namespace


struct IsoTimeZoneOptions {
  IsoTimeZoneFormat format;
  IsoTimeZoneMinuteDisplay minutes;
  IsoTimeZoneSecondDisplay seconds;

  inline diplomat::capi::IsoTimeZoneOptions AsFFI() const;
  inline static IsoTimeZoneOptions FromFFI(diplomat::capi::IsoTimeZoneOptions c_struct);
};


#endif // IsoTimeZoneOptions_D_HPP
