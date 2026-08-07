#ifndef TimeZone_D_H
#define TimeZone_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TimeZone TimeZone;


typedef struct DiplomatTimeZoneView {
  const TimeZone** data;
  size_t len;
} DiplomatTimeZoneView;



#endif // TimeZone_D_H
