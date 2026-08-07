#ifndef IsoDate_D_H
#define IsoDate_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct IsoDate IsoDate;


typedef struct DiplomatIsoDateView {
  const IsoDate** data;
  size_t len;
} DiplomatIsoDateView;



#endif // IsoDate_D_H
