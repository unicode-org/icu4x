#ifndef Date_D_H
#define Date_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Date Date;


typedef struct DiplomatDateView {
  const Date** data;
  size_t len;
} DiplomatDateView;



#endif // Date_D_H
