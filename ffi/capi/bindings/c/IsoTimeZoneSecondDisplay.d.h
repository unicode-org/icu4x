#ifndef IsoTimeZoneSecondDisplay_D_H
#define IsoTimeZoneSecondDisplay_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum IsoTimeZoneSecondDisplay {
  IsoTimeZoneSecondDisplay_Optional = 0,
  IsoTimeZoneSecondDisplay_Never = 1,
} IsoTimeZoneSecondDisplay;

typedef struct IsoTimeZoneSecondDisplay_option {union { IsoTimeZoneSecondDisplay ok; }; bool is_ok; } IsoTimeZoneSecondDisplay_option;



#endif // IsoTimeZoneSecondDisplay_D_H
