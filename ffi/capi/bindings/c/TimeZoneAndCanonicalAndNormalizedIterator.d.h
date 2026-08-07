#ifndef TimeZoneAndCanonicalAndNormalizedIterator_D_H
#define TimeZoneAndCanonicalAndNormalizedIterator_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef struct TimeZoneAndCanonicalAndNormalizedIterator TimeZoneAndCanonicalAndNormalizedIterator;


typedef struct DiplomatTimeZoneAndCanonicalAndNormalizedIteratorView {
  const TimeZoneAndCanonicalAndNormalizedIterator** data;
  size_t len;
} DiplomatTimeZoneAndCanonicalAndNormalizedIteratorView;



#endif // TimeZoneAndCanonicalAndNormalizedIterator_D_H
