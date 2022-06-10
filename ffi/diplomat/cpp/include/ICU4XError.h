#ifndef ICU4XError_H
#define ICU4XError_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ICU4XError {
  ICU4XError_UnknownError = 0,
  ICU4XError_WriteableError = 1,
  ICU4XError_OutOfBoundsError = 2,
  ICU4XError_DataMissingResourceKeyError = 10,
  ICU4XError_DataMissingVariantError = 11,
  ICU4XError_DataMissingLocaleError = 12,
  ICU4XError_DataMissingResourceOptionsError = 13,
  ICU4XError_DataNeedsVariantError = 14,
  ICU4XError_DataNeedsLocaleError = 15,
  ICU4XError_DataExtraneousResourceOptionsError = 16,
  ICU4XError_DataFilteredResourceError = 17,
  ICU4XError_DataMismatchedTypeError = 18,
  ICU4XError_DataMissingPayloadError = 19,
  ICU4XError_DataInvalidStateError = 20,
  ICU4XError_DataCustomError = 21,
  ICU4XError_DataIoError = 22,
  ICU4XError_DataUnavailableBufferFormatError = 23,
  ICU4XError_LocaleUndefinedSubtagError = 31,
  ICU4XError_LocaleParserError = 32,
  ICU4XError_DataStructValidityError = 33,
  ICU4XError_PropertyUnknownScriptIdError = 40,
  ICU4XError_PropertyUnknownGeneralCategoryGroupError = 41,
  ICU4XError_DecimalLimit = 42,
  ICU4XError_DecimalSyntax = 43,
} ICU4XError;

void ICU4XError_destroy(ICU4XError* self);

#ifdef __cplusplus
}
#endif
#endif
