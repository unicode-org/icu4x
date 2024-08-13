#ifndef CollatorStrength_D_H
#define CollatorStrength_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorStrength {
  CollatorStrength_Auto = 0,
  CollatorStrength_Primary = 1,
  CollatorStrength_Secondary = 2,
  CollatorStrength_Tertiary = 3,
  CollatorStrength_Quaternary = 4,
  CollatorStrength_Identical = 5,
} CollatorStrength;





#endif // CollatorStrength_D_H
