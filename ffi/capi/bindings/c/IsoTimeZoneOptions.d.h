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

typedef struct IsoTimeZoneOptions_option {union { IsoTimeZoneOptions ok; }; bool is_ok; } IsoTimeZoneOptions_option;



#endif // IsoTimeZoneOptions_D_H
