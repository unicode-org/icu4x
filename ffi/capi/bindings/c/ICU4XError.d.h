#ifndef ICU4XError_D_H
#define ICU4XError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XError {
  ICU4XError_UnknownError = 0,
  ICU4XError_DataMissingDataMarkerError = 256,
  ICU4XError_DataMissingVariantError = 257,
  ICU4XError_DataMissingLocaleError = 258,
  ICU4XError_DataNeedsVariantError = 259,
  ICU4XError_DataNeedsLocaleError = 260,
  ICU4XError_DataExtraneousLocaleError = 261,
  ICU4XError_DataFilteredResourceError = 262,
  ICU4XError_DataMismatchedTypeError = 263,
  ICU4XError_DataMissingPayloadError = 264,
  ICU4XError_DataInvalidStateError = 265,
  ICU4XError_DataCustomError = 266,
  ICU4XError_DataIoError = 267,
  ICU4XError_DataUnavailableBufferFormatError = 268,
  ICU4XError_DataMismatchedAnyBufferError = 269,
  ICU4XError_PropertyUnexpectedPropertyNameError = 1026,
  ICU4XError_DateTimePatternError = 2048,
  ICU4XError_DateTimeMissingInputFieldError = 2049,
  ICU4XError_DateTimeSkeletonError = 2050,
  ICU4XError_DateTimeUnsupportedFieldError = 2051,
  ICU4XError_DateTimeUnsupportedOptionsError = 2052,
  ICU4XError_DateTimeMissingWeekdaySymbolError = 2053,
  ICU4XError_DateTimeMissingMonthSymbolError = 2054,
  ICU4XError_DateTimeFixedDecimalError = 2055,
  ICU4XError_DateTimeMismatchedCalendarError = 2056,
} ICU4XError;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XError_D_H
