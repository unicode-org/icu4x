#ifndef Error_D_H
#define Error_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum Error {
  Error_UnknownError = 0,
  Error_DataMissingDataMarkerError = 256,
  Error_DataMissingLocaleError = 258,
  Error_DataNeedsLocaleError = 260,
  Error_DataExtraneousLocaleError = 261,
  Error_DataFilteredResourceError = 262,
  Error_DataMismatchedTypeError = 263,
  Error_DataCustomError = 266,
  Error_DataIoError = 267,
  Error_DataUnavailableBufferFormatError = 268,
  Error_PropertyUnexpectedPropertyNameError = 1026,
  Error_DateTimePatternError = 2048,
  Error_DateTimeMissingInputFieldError = 2049,
  Error_DateTimeSkeletonError = 2050,
  Error_DateTimeUnsupportedFieldError = 2051,
  Error_DateTimeUnsupportedOptionsError = 2052,
  Error_DateTimeMissingWeekdaySymbolError = 2053,
  Error_DateTimeMissingMonthSymbolError = 2054,
  Error_DateTimeFixedDecimalError = 2055,
  Error_DateTimeMismatchedCalendarError = 2056,
} Error;

typedef struct Error_option {union { Error ok; }; bool is_ok; } Error_option;



#endif // Error_D_H
