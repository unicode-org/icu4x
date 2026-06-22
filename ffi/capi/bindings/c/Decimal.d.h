#ifndef Decimal_D_H
#define Decimal_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Decimal Decimal;


typedef struct DiplomatDecimalView {
  const Decimal** data;
  size_t len;
} DiplomatDecimalView;



#endif // Decimal_D_H
