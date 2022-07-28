#ifndef ICU4XError_H
#define ICU4XError_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef enum ICU4XError {
  ICU4XError_UnknownError = 0,
  ICU4XError_WriteableError = 1,
  ICU4XError_OutOfBoundsError = 2,
  ICU4XError_DataMissingDataKeyError = 256,
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
  ICU4XError_LocaleUndefinedSubtagError = 512,
  ICU4XError_LocaleParserError = 513,
  ICU4XError_DataStructValidityError = 768,
  ICU4XError_PropertyUnknownScriptIdError = 1024,
  ICU4XError_PropertyUnknownGeneralCategoryGroupError = 1025,
  ICU4XError_DecimalLimitError = 1280,
  ICU4XError_DecimalSyntaxError = 1281,
  ICU4XError_PluralParserError = 1536,
  ICU4XError_DateTimeParseError = 1792,
  ICU4XError_DateTimeOverflowError = 1793,
  ICU4XError_DateTimeUnderflowError = 1794,
  ICU4XError_DateTimeOutOfRangeError = 1795,
  ICU4XError_DateTimeMissingInputError = 1796,
  ICU4XError_DateTimeFormatPatternError = 2048,
  ICU4XError_DateTimeFormatMissingInputFieldError = 2049,
  ICU4XError_DateTimeFormatSkeletonError = 2050,
  ICU4XError_DateTimeFormatUnsupportedFieldError = 2051,
  ICU4XError_DateTimeFormatUnsupportedOptionsError = 2052,
  ICU4XError_DateTimeFormatMissingWeekdaySymbolError = 2053,
  ICU4XError_DateTimeFormatMissingMonthSymbolError = 2054,
  ICU4XError_DateTimeFormatFixedDecimalError = 2055,
  ICU4XError_DateTimeFormatMismatchedAnyCalendarError = 2056,
  ICU4XError_DateTimeFormatMismatchedCalendarLocaleError = 2057,
} ICU4XError;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XError_destroy(ICU4XError* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
