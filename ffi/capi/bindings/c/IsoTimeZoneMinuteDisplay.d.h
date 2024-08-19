#ifndef IsoTimeZoneMinuteDisplay_D_H
#define IsoTimeZoneMinuteDisplay_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum IsoTimeZoneMinuteDisplay {
  IsoTimeZoneMinuteDisplay_Required = 0,
  IsoTimeZoneMinuteDisplay_Optional = 1,
} IsoTimeZoneMinuteDisplay;

typedef struct IsoTimeZoneMinuteDisplay_option {union { IsoTimeZoneMinuteDisplay ok; }; bool is_ok; } IsoTimeZoneMinuteDisplay_option;



#endif // IsoTimeZoneMinuteDisplay_D_H
