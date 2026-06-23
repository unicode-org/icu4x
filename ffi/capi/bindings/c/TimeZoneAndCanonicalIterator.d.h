#ifndef TimeZoneAndCanonicalIterator_D_H
#define TimeZoneAndCanonicalIterator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TimeZoneAndCanonicalIterator TimeZoneAndCanonicalIterator;


typedef struct DiplomatTimeZoneAndCanonicalIteratorView {
  const TimeZoneAndCanonicalIterator** data;
  size_t len;
} DiplomatTimeZoneAndCanonicalIteratorView;



#endif // TimeZoneAndCanonicalIterator_D_H
