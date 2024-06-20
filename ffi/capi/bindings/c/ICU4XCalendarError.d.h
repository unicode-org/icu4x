#ifndef ICU4XCalendarError_D_H
#define ICU4XCalendarError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XCalendarError {
  ICU4XCalendarError_Unknown = 0,
  ICU4XCalendarError_OutOfRange = 1,
  ICU4XCalendarError_UnknownEra = 2,
  ICU4XCalendarError_UnknownMonthCode = 3,
} ICU4XCalendarError;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCalendarError_D_H
