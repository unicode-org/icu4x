#ifndef TimeZoneIterator_D_H
#define TimeZoneIterator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TimeZoneIterator TimeZoneIterator;


typedef struct DiplomatTimeZoneIteratorView {
  const TimeZoneIterator** data;
  size_t len;
} DiplomatTimeZoneIteratorView;



#endif // TimeZoneIterator_D_H
