#ifndef CalendarDateDifferenceError_D_H
#define CalendarDateDifferenceError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum CalendarDateDifferenceError {
  CalendarDateDifferenceError_Unknown = 0,
  CalendarDateDifferenceError_MismatchedCalendars = 1,
} CalendarDateDifferenceError;

typedef struct CalendarDateDifferenceError_option {union { CalendarDateDifferenceError ok; }; bool is_ok; } CalendarDateDifferenceError_option;



#endif // CalendarDateDifferenceError_D_H
