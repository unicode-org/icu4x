#ifndef FromIxdtfError_D_H
#define FromIxdtfError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum FromIxdtfError {
  FromIxdtfError_Unknown = 0,
  FromIxdtfError_InvalidSyntax = 1,
  FromIxdtfError_OutOfRange = 2,
  FromIxdtfError_MissingFields = 3,
  FromIxdtfError_UnknownCalendar = 4,
} FromIxdtfError;





#endif // FromIxdtfError_D_H
