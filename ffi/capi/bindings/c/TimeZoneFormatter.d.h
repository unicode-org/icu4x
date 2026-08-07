#ifndef TimeZoneFormatter_D_H
#define TimeZoneFormatter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TimeZoneFormatter TimeZoneFormatter;


typedef struct DiplomatTimeZoneFormatterView {
  const TimeZoneFormatter** data;
  size_t len;
} DiplomatTimeZoneFormatterView;



#endif // TimeZoneFormatter_D_H
