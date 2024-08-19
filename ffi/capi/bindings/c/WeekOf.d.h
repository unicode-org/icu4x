#ifndef WeekOf_D_H
#define WeekOf_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "WeekRelativeUnit.d.h"




typedef struct WeekOf {
  uint16_t week;
  WeekRelativeUnit unit;
} WeekOf;





#endif // WeekOf_D_H
