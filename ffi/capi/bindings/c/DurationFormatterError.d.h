#ifndef DurationFormatterError_D_H
#define DurationFormatterError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DurationFormatterError {
  DurationFormatterError_FractionalDigitsOutOfRange = 0,
  DurationFormatterError_PreviousNumeric = 1,
  DurationFormatterError_DisplayAlwaysFractional = 2,
  DurationFormatterError_PreviousFractional = 3,
  DurationFormatterError_InvalidUnitStyle = 4,
  DurationFormatterError_DataError = 5,
} DurationFormatterError;

typedef struct DurationFormatterError_option {union { DurationFormatterError ok; }; bool is_ok; } DurationFormatterError_option;



#endif // DurationFormatterError_D_H
