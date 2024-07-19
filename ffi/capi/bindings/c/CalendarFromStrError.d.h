#ifndef CalendarFromStrError_D_H
#define CalendarFromStrError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CalendarFromStrError {
  CalendarFromStrError_Unknown = 0,
  CalendarFromStrError_InvalidSyntax = 1,
  CalendarFromStrError_OutOfRange = 2,
  CalendarFromStrError_MissingFields = 3,
  CalendarFromStrError_UnknownCalendar = 4,
} CalendarFromStrError;





#endif // CalendarFromStrError_D_H
