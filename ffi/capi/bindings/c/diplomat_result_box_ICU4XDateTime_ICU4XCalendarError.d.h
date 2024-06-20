#ifndef diplomat_result_box_ICU4XDateTime_ICU4XCalendarError_D_H
#define diplomat_result_box_ICU4XDateTime_ICU4XCalendarError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCalendarError.d.h"
#include "ICU4XDateTime.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct diplomat_result_box_ICU4XDateTime_ICU4XCalendarError {
  union {
    ICU4XDateTime* ok;
    ICU4XCalendarError err;
  };
  bool is_ok;
} diplomat_result_box_ICU4XDateTime_ICU4XCalendarError;

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // diplomat_result_box_ICU4XDateTime_ICU4XCalendarError_D_H
