#ifndef diplomat_result_box_ICU4XCustomTimeZone_ICU4XTimeZoneInvalidOffsetError_D_H
#define diplomat_result_box_ICU4XCustomTimeZone_ICU4XTimeZoneInvalidOffsetError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCustomTimeZone.d.h"
#include "ICU4XTimeZoneInvalidOffsetError.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct diplomat_result_box_ICU4XCustomTimeZone_ICU4XTimeZoneInvalidOffsetError {
  union {
    ICU4XCustomTimeZone* ok;
    ICU4XTimeZoneInvalidOffsetError err;
  };
  bool is_ok;
} diplomat_result_box_ICU4XCustomTimeZone_ICU4XTimeZoneInvalidOffsetError;

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // diplomat_result_box_ICU4XCustomTimeZone_ICU4XTimeZoneInvalidOffsetError_D_H
