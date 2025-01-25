#ifndef ZoneStyle_D_H
#define ZoneStyle_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ZoneStyle {
  ZoneStyle_Z = 0,
  ZoneStyle_Zs = 1,
  ZoneStyle_O = 2,
  ZoneStyle_Os = 3,
  ZoneStyle_V = 4,
  ZoneStyle_Vs = 5,
  ZoneStyle_L = 6,
} ZoneStyle;

typedef struct ZoneStyle_option {union { ZoneStyle ok; }; bool is_ok; } ZoneStyle_option;



#endif // ZoneStyle_D_H
