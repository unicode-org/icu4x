#ifndef ZonedTimeFormatter_D_H
#define ZonedTimeFormatter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ZonedTimeFormatter ZonedTimeFormatter;


typedef struct DiplomatZonedTimeFormatterView {
  const ZonedTimeFormatter** data;
  size_t len;
} DiplomatZonedTimeFormatterView;



#endif // ZonedTimeFormatter_D_H
