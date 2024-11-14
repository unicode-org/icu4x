#ifndef PatternLoadError_D_H
#define PatternLoadError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"





typedef enum PatternLoadError {
  PatternLoadError_Unknown = 0,
  PatternLoadError_UnsupportedLength = 2051,
  PatternLoadError_DuplicateField = 2057,
  PatternLoadError_TypeTooSpecific = 2058,
  PatternLoadError_DataMarkerNotFound = 1,
  PatternLoadError_DataIdentifierNotFound = 2,
  PatternLoadError_DataInvalidRequest = 3,
  PatternLoadError_DataInconsistentData = 4,
  PatternLoadError_DataDowncast = 5,
  PatternLoadError_DataDeserialize = 6,
  PatternLoadError_DataCustom = 7,
  PatternLoadError_DataIo = 8,
} PatternLoadError;

typedef struct PatternLoadError_option {union { PatternLoadError ok; }; bool is_ok; } PatternLoadError_option;



#endif // PatternLoadError_D_H
