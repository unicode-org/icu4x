#ifndef diplomat_result_box_ICU4XDateTimeFormatter_ICU4XError_D_H
#define diplomat_result_box_ICU4XDateTimeFormatter_ICU4XError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDateTimeFormatter.d.h"
#include "ICU4XError.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct diplomat_result_box_ICU4XDateTimeFormatter_ICU4XError {
  union {
    ICU4XDateTimeFormatter* ok;
    ICU4XError err;
  };
  bool is_ok;
} diplomat_result_box_ICU4XDateTimeFormatter_ICU4XError;

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // diplomat_result_box_ICU4XDateTimeFormatter_ICU4XError_D_H
