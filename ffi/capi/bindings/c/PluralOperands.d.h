#ifndef PluralOperands_D_H
#define PluralOperands_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct PluralOperands PluralOperands;


typedef struct DiplomatPluralOperandsView {
  const PluralOperands** data;
  size_t len;
} DiplomatPluralOperandsView;



#endif // PluralOperands_D_H
