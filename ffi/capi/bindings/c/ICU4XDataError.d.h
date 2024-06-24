#ifndef ICU4XDataError_D_H
#define ICU4XDataError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ICU4XDataError {
  ICU4XDataError_Unknown = 0,
  ICU4XDataError_MissingDataMarker = 1,
  ICU4XDataError_MissingLocale = 2,
  ICU4XDataError_NeedsLocale = 3,
  ICU4XDataError_ExtraneousLocale = 4,
  ICU4XDataError_FilteredResource = 5,
  ICU4XDataError_MismatchedType = 6,
  ICU4XDataError_Custom = 7,
  ICU4XDataError_Io = 8,
  ICU4XDataError_UnavailableBufferFormat = 9,
  ICU4XDataError_InconsistentData = 10,
} ICU4XDataError;





#endif // ICU4XDataError_D_H
