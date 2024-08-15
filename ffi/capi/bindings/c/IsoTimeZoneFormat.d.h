#ifndef IsoTimeZoneFormat_D_H
#define IsoTimeZoneFormat_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum IsoTimeZoneFormat {
  IsoTimeZoneFormat_Basic = 0,
  IsoTimeZoneFormat_Extended = 1,
  IsoTimeZoneFormat_UtcBasic = 2,
  IsoTimeZoneFormat_UtcExtended = 3,
} IsoTimeZoneFormat;

typedef struct IsoTimeZoneFormat_option {union { IsoTimeZoneFormat ok; }; bool is_ok; } IsoTimeZoneFormat_option;



#endif // IsoTimeZoneFormat_D_H
