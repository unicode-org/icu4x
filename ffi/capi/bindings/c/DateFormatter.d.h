#ifndef DateFormatter_D_H
#define DateFormatter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct DateFormatter DateFormatter;


typedef struct DiplomatDateFormatterView {
  const DateFormatter** data;
  size_t len;
} DiplomatDateFormatterView;



#endif // DateFormatter_D_H
