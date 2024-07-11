#ifndef FixedDecimalParseError_D_H
#define FixedDecimalParseError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum FixedDecimalParseError {
  FixedDecimalParseError_Unknown = 0,
  FixedDecimalParseError_Limit = 1,
  FixedDecimalParseError_Syntax = 2,
} FixedDecimalParseError;





#endif // FixedDecimalParseError_D_H
