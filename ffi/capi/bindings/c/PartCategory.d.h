#ifndef PartCategory_D_H
#define PartCategory_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum PartCategory {
  PartCategory_Unknown = 0,
  PartCategory_Error = 1,
  PartCategory_List = 2,
  PartCategory_Decimal = 3,
  PartCategory_DateTime = 4,
  PartCategory_RelativeTime = 5,
  PartCategory_Duration = 6,
  PartCategory_Unit = 7,
} PartCategory;

typedef struct PartCategory_option {union { PartCategory ok; }; bool is_ok; } PartCategory_option;



#endif // PartCategory_D_H
