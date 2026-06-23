#ifndef TimeFormatter_D_H
#define TimeFormatter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TimeFormatter TimeFormatter;


typedef struct DiplomatTimeFormatterView {
  const TimeFormatter** data;
  size_t len;
} DiplomatTimeFormatterView;



#endif // TimeFormatter_D_H
