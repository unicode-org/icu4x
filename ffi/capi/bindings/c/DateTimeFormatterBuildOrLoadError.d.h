#ifndef DateTimeFormatterBuildOrLoadError_D_H
#define DateTimeFormatterBuildOrLoadError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum DateTimeFormatterBuildOrLoadError {
  DateTimeFormatterBuildOrLoadError_Unknown = 0,
  DateTimeFormatterBuildOrLoadError_DataMarkerNotFound = 1,
  DateTimeFormatterBuildOrLoadError_DataIdentifierNotFound = 2,
  DateTimeFormatterBuildOrLoadError_DataInvalidRequest = 3,
  DateTimeFormatterBuildOrLoadError_DataInconsistentData = 4,
  DateTimeFormatterBuildOrLoadError_DataDowncast = 5,
  DateTimeFormatterBuildOrLoadError_DataDeserialize = 6,
  DateTimeFormatterBuildOrLoadError_DataCustom = 7,
  DateTimeFormatterBuildOrLoadError_DataIo = 8,
  DateTimeFormatterBuildOrLoadError_MissingDateFields = 1025,
  DateTimeFormatterBuildOrLoadError_MissingTimePrecision = 1026,
  DateTimeFormatterBuildOrLoadError_MissingZoneStyle = 1027,
  DateTimeFormatterBuildOrLoadError_InvalidDateFields = 1028,
  DateTimeFormatterBuildOrLoadError_SuperfluousOptions = 1029,
  DateTimeFormatterBuildOrLoadError_UnsupportedLength = 2051,
  DateTimeFormatterBuildOrLoadError_ConflictingField = 2057,
  DateTimeFormatterBuildOrLoadError_FormatterTooSpecific = 2058,
} DateTimeFormatterBuildOrLoadError;

typedef struct DateTimeFormatterBuildOrLoadError_option {union { DateTimeFormatterBuildOrLoadError ok; }; bool is_ok; } DateTimeFormatterBuildOrLoadError_option;



#endif // DateTimeFormatterBuildOrLoadError_D_H
