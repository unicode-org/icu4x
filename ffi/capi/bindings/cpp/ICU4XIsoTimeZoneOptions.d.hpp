#ifndef ICU4XIsoTimeZoneOptions_D_HPP
#define ICU4XIsoTimeZoneOptions_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XIsoTimeZoneFormat.d.hpp"
#include "ICU4XIsoTimeZoneMinuteDisplay.d.hpp"
#include "ICU4XIsoTimeZoneSecondDisplay.d.hpp"

class ICU4XIsoTimeZoneFormat;
class ICU4XIsoTimeZoneMinuteDisplay;
class ICU4XIsoTimeZoneSecondDisplay;


namespace capi {
    typedef struct ICU4XIsoTimeZoneOptions {
      ICU4XIsoTimeZoneFormat format;
      ICU4XIsoTimeZoneMinuteDisplay minutes;
      ICU4XIsoTimeZoneSecondDisplay seconds;
    } ICU4XIsoTimeZoneOptions;
}

struct ICU4XIsoTimeZoneOptions {
  ICU4XIsoTimeZoneFormat format;
  ICU4XIsoTimeZoneMinuteDisplay minutes;
  ICU4XIsoTimeZoneSecondDisplay seconds;

  inline capi::ICU4XIsoTimeZoneOptions AsFFI() const;
  inline static ICU4XIsoTimeZoneOptions FromFFI(capi::ICU4XIsoTimeZoneOptions c_struct);
};


#endif // ICU4XIsoTimeZoneOptions_D_HPP
