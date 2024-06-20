#ifndef ICU4XLocaleParseError_D_H
#define ICU4XLocaleParseError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XLocaleParseError {
  ICU4XLocaleParseError_Unknown = 0,
  ICU4XLocaleParseError_Language = 1,
  ICU4XLocaleParseError_Subtag = 2,
  ICU4XLocaleParseError_Extension = 3,
} ICU4XLocaleParseError;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocaleParseError_D_H
