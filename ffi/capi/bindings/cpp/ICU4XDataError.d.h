#ifndef ICU4XDataError_D_H
#define ICU4XDataError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XDataError {
  ICU4XDataError_Unknown = 0,
  ICU4XDataError_MissingDataMarker = 1,
  ICU4XDataError_MissingVariant = 2,
  ICU4XDataError_MissingLocale = 3,
  ICU4XDataError_NeedsVariant = 4,
  ICU4XDataError_NeedsLocale = 5,
  ICU4XDataError_ExtraneousLocale = 6,
  ICU4XDataError_FilteredResource = 7,
  ICU4XDataError_MismatchedType = 8,
  ICU4XDataError_MissingPayload = 9,
  ICU4XDataError_InvalidState = 10,
  ICU4XDataError_Custom = 11,
  ICU4XDataError_Io = 12,
  ICU4XDataError_UnavailableBufferFormat = 13,
  ICU4XDataError_MismatchedAnyBuffer = 14,
  ICU4XDataError_DataStructValidityError = 15,
} ICU4XDataError;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XDataError_D_H
