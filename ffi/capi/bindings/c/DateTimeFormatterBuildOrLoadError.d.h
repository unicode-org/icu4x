#ifndef DateTimeFormatterBuildOrLoadError_D_H
#define DateTimeFormatterBuildOrLoadError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DateTimeFormatterBuildOrLoadError {
  DateTimeFormatterBuildOrLoadError_Unknown = 0,
  DateTimeFormatterBuildOrLoadError_InvalidFields = 256,
  DateTimeFormatterBuildOrLoadError_InvalidOptions = 257,
  DateTimeFormatterBuildOrLoadError_UnsupportedLength = 2051,
  DateTimeFormatterBuildOrLoadError_DuplicateField = 2057,
  DateTimeFormatterBuildOrLoadError_TypeTooSpecific = 2058,
  DateTimeFormatterBuildOrLoadError_DataMarkerNotFound = 1,
  DateTimeFormatterBuildOrLoadError_DataIdentifierNotFound = 2,
  DateTimeFormatterBuildOrLoadError_DataInvalidRequest = 3,
  DateTimeFormatterBuildOrLoadError_DataInconsistentData = 4,
  DateTimeFormatterBuildOrLoadError_DataDowncast = 5,
  DateTimeFormatterBuildOrLoadError_DataDeserialize = 6,
  DateTimeFormatterBuildOrLoadError_DataCustom = 7,
  DateTimeFormatterBuildOrLoadError_DataIo = 8,
} DateTimeFormatterBuildOrLoadError;

typedef struct DateTimeFormatterBuildOrLoadError_option {union { DateTimeFormatterBuildOrLoadError ok; }; bool is_ok; } DateTimeFormatterBuildOrLoadError_option;



#endif // DateTimeFormatterBuildOrLoadError_D_H
