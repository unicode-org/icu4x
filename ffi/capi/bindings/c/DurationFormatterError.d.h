#ifndef DurationFormatterError_D_H
#define DurationFormatterError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DurationFormatterError {
  DurationFormatterError_Unknown = 0,
  DurationFormatterError_FractionalDigitsOutOfRange = 2049,
  DurationFormatterError_PreviousNumeric = 2050,
  DurationFormatterError_DisplayAlwaysFractional = 2051,
  DurationFormatterError_PreviousFractional = 2052,
  DurationFormatterError_InvalidUnitStyle = 2053,
  DurationFormatterError_DataMarkerNotFound = 1,
  DurationFormatterError_DataIdentifierNotFound = 2,
  DurationFormatterError_DataInvalidRequest = 3,
  DurationFormatterError_DataInconsistentData = 4,
  DurationFormatterError_DataDowncast = 5,
  DurationFormatterError_DataDeserialize = 6,
  DurationFormatterError_DataCustom = 7,
  DurationFormatterError_DataIo = 8,
} DurationFormatterError;

typedef struct DurationFormatterError_option {union { DurationFormatterError ok; }; bool is_ok; } DurationFormatterError_option;



#endif // DurationFormatterError_D_H
