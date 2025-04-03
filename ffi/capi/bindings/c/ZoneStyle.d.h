#ifndef ZoneStyle_D_H
#define ZoneStyle_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ZoneStyle {
  ZoneStyle_SpecificLong = 0,
  ZoneStyle_SpecificShort = 1,
  ZoneStyle_LocalizedOffsetLong = 2,
  ZoneStyle_LocalizedOffsetShort = 3,
  ZoneStyle_GenericLong = 4,
  ZoneStyle_GenericShort = 5,
  ZoneStyle_Location = 6,
  ZoneStyle_ExemplarCity = 7,
} ZoneStyle;

typedef struct ZoneStyle_option {union { ZoneStyle ok; }; bool is_ok; } ZoneStyle_option;



#endif // ZoneStyle_D_H
