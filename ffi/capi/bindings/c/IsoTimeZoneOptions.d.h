#ifndef IsoTimeZoneOptions_D_H
#define IsoTimeZoneOptions_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "IsoTimeZoneFormat.d.h"
#include "IsoTimeZoneMinuteDisplay.d.h"
#include "IsoTimeZoneSecondDisplay.d.h"




typedef struct IsoTimeZoneOptions {
  IsoTimeZoneFormat format;
  IsoTimeZoneMinuteDisplay minutes;
  IsoTimeZoneSecondDisplay seconds;
} IsoTimeZoneOptions;





#endif // IsoTimeZoneOptions_D_H
