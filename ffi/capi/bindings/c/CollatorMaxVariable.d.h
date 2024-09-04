#ifndef CollatorMaxVariable_D_H
#define CollatorMaxVariable_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CollatorMaxVariable {
  CollatorMaxVariable_Auto = 0,
  CollatorMaxVariable_Space = 1,
  CollatorMaxVariable_Punctuation = 2,
  CollatorMaxVariable_Symbol = 3,
  CollatorMaxVariable_Currency = 4,
} CollatorMaxVariable;

typedef struct CollatorMaxVariable_option {union { CollatorMaxVariable ok; }; bool is_ok; } CollatorMaxVariable_option;



#endif // CollatorMaxVariable_D_H
