#ifndef WeekdaySetIterator_D_H
#define WeekdaySetIterator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct WeekdaySetIterator WeekdaySetIterator;


typedef struct DiplomatWeekdaySetIteratorView {
  const WeekdaySetIterator** data;
  size_t len;
} DiplomatWeekdaySetIteratorView;



#endif // WeekdaySetIterator_D_H
