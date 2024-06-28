#ifndef ICU4XDataError_D_H
#define ICU4XDataError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum ICU4XDataError {
  ICU4XDataError_Unknown = 0,
  ICU4XDataError_MarkerNotFound = 1,
  ICU4XDataError_IdentifierNotFound = 2,
  ICU4XDataError_InvalidRequest = 3,
  ICU4XDataError_FilteredResource = 4,
  ICU4XDataError_InconsistentData = 5,
  ICU4XDataError_Downcast = 6,
  ICU4XDataError_Deserialize = 7,
  ICU4XDataError_Custom = 8,
  ICU4XDataError_Io = 9,
} ICU4XDataError;





#endif // ICU4XDataError_D_H
