#ifndef DateTimeWriteError_D_H
#define DateTimeWriteError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DateTimeWriteError {
  DateTimeWriteError_Unknown = 0,
  DateTimeWriteError_InvalidMonthCode = 2,
  DateTimeWriteError_InvalidEra = 3,
  DateTimeWriteError_InvalidCyclicYear = 4,
  DateTimeWriteError_DecimalFormatterNotLoaded = 5,
  DateTimeWriteError_NamesNotLoaded = 6,
  DateTimeWriteError_MissingInputField = 7,
  DateTimeWriteError_UnsupportedLength = 8,
  DateTimeWriteError_UnsupportedField = 9,
} DateTimeWriteError;

typedef struct DateTimeWriteError_option {union { DateTimeWriteError ok; }; bool is_ok; } DateTimeWriteError_option;



#endif // DateTimeWriteError_D_H
