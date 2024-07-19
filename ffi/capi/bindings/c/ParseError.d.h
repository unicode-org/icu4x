#ifndef ParseError_D_H
#define ParseError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ParseError {
  ParseError_Unknown = 0,
  ParseError_InvalidSyntax = 1,
  ParseError_OutOfRange = 2,
  ParseError_MissingFields = 3,
  ParseError_UnknownCalendar = 4,
} ParseError;





#endif // ParseError_D_H
