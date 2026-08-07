#ifndef Time_D_H
#define Time_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct Time Time;


typedef struct DiplomatTimeView {
  const Time** data;
  size_t len;
} DiplomatTimeView;



#endif // Time_D_H
