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
  ICU4XError_LocaleUndefinedSubtagError = 2,
  ICU4XError_LocaleParserError = 3,
  ICU4XError_DataStructValidityError = 4,
  ICU4XError_DataMissingResourceKeyError = 5,
  ICU4XError_DataMissingVariantError = 6,
  ICU4XError_DataMissingLocaleError = 7,
  ICU4XError_DataMissingResourceOptionsError = 8,
  ICU4XError_DataNeedsVariantError = 9,
  ICU4XError_DataNeedsLocaleError = 10,
  ICU4XError_DataExtraneousResourceOptionsError = 11,
  ICU4XError_DataFilteredResourceError = 12,
  ICU4XError_DataMismatchedTypeError = 13,
  ICU4XError_DataMissingPayloadError = 14,
  ICU4XError_DataInvalidStateError = 15,
  ICU4XError_DataCustomError = 16,
  ICU4XError_DataIoError = 17,
  ICU4XError_DataUnavailableBufferFormatError = 18,
  ICU4XError_PropertyUnknownScriptIdError = 19,
  ICU4XError_PropertyUnknownGeneralCategoryGroupError = 20,
} ICU4XError;

void ICU4XError_destroy(ICU4XError* self);

#ifdef __cplusplus
}
#endif
#endif
