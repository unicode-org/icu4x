#ifndef ZonedDateFormatter_D_H
#define ZonedDateFormatter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct ZonedDateFormatter ZonedDateFormatter;


typedef struct DiplomatZonedDateFormatterView {
  const ZonedDateFormatter** data;
  size_t len;
} DiplomatZonedDateFormatterView;



#endif // ZonedDateFormatter_D_H
