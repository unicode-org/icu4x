#ifndef ICU4XIsoTimeZoneOptions_D_H
#define ICU4XIsoTimeZoneOptions_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XIsoTimeZoneFormat.d.h"
#include "ICU4XIsoTimeZoneMinuteDisplay.d.h"
#include "ICU4XIsoTimeZoneSecondDisplay.d.h"




typedef struct ICU4XIsoTimeZoneOptions {
  ICU4XIsoTimeZoneFormat format;
  ICU4XIsoTimeZoneMinuteDisplay minutes;
  ICU4XIsoTimeZoneSecondDisplay seconds;
} ICU4XIsoTimeZoneOptions;





#endif // ICU4XIsoTimeZoneOptions_D_H
