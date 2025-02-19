#ifndef TimeZoneAndCanonicalAndNormalized_D_H
#define TimeZoneAndCanonicalAndNormalized_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "TimeZoneInfo.d.h"




typedef struct TimeZoneAndCanonicalAndNormalized {
  TimeZoneInfo* time_zone;
  DiplomatStringView canonical;
  DiplomatStringView normalized;
} TimeZoneAndCanonicalAndNormalized;

typedef struct TimeZoneAndCanonicalAndNormalized_option {union { TimeZoneAndCanonicalAndNormalized ok; }; bool is_ok; } TimeZoneAndCanonicalAndNormalized_option;



#endif // TimeZoneAndCanonicalAndNormalized_D_H
