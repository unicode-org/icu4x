#ifndef CollatorBackwardSecondLevel_D_H
#define CollatorBackwardSecondLevel_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorBackwardSecondLevel {
  CollatorBackwardSecondLevel_Auto = 0,
  CollatorBackwardSecondLevel_Off = 1,
  CollatorBackwardSecondLevel_On = 2,
} CollatorBackwardSecondLevel;

typedef struct CollatorBackwardSecondLevel_option {union { CollatorBackwardSecondLevel ok; }; bool is_ok; } CollatorBackwardSecondLevel_option;



#endif // CollatorBackwardSecondLevel_D_H
