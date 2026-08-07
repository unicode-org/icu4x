#ifndef DecimalFormatter_D_H
#define DecimalFormatter_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct DecimalFormatter DecimalFormatter;


typedef struct DiplomatDecimalFormatterView {
  const DecimalFormatter** data;
  size_t len;
} DiplomatDecimalFormatterView;



#endif // DecimalFormatter_D_H
