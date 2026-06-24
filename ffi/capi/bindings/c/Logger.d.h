#ifndef Logger_D_H
#define Logger_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Logger Logger;


typedef struct DiplomatLoggerView {
  const Logger** data;
  size_t len;
} DiplomatLoggerView;



#endif // Logger_D_H
