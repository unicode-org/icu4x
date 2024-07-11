#ifndef IsoTimeZoneOptions_D_HPP
#define IsoTimeZoneOptions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "IsoTimeZoneFormat.d.hpp"
#include "IsoTimeZoneMinuteDisplay.d.hpp"
#include "IsoTimeZoneSecondDisplay.d.hpp"

class IsoTimeZoneFormat;
class IsoTimeZoneMinuteDisplay;
class IsoTimeZoneSecondDisplay;


namespace capi {
    typedef struct IsoTimeZoneOptions {
      IsoTimeZoneFormat format;
      IsoTimeZoneMinuteDisplay minutes;
      IsoTimeZoneSecondDisplay seconds;
    } IsoTimeZoneOptions;
}

struct IsoTimeZoneOptions {
  IsoTimeZoneFormat format;
  IsoTimeZoneMinuteDisplay minutes;
  IsoTimeZoneSecondDisplay seconds;

  inline capi::IsoTimeZoneOptions AsFFI() const;
  inline static IsoTimeZoneOptions FromFFI(capi::IsoTimeZoneOptions c_struct);
};


#endif // IsoTimeZoneOptions_D_HPP
