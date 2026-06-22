#ifndef TimeZoneInfo_D_H
#define TimeZoneInfo_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TimeZoneInfo TimeZoneInfo;


typedef struct DiplomatTimeZoneInfoView {
  const TimeZoneInfo** data;
  size_t len;
} DiplomatTimeZoneInfoView;



#endif // TimeZoneInfo_D_H
