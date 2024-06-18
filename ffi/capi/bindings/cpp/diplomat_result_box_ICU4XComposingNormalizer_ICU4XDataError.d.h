#ifndef diplomat_result_box_ICU4XComposingNormalizer_ICU4XDataError_D_H
#define diplomat_result_box_ICU4XComposingNormalizer_ICU4XDataError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XComposingNormalizer.d.h"
#include "ICU4XDataError.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct diplomat_result_box_ICU4XComposingNormalizer_ICU4XDataError {
  union {
    ICU4XComposingNormalizer* ok;
    ICU4XDataError err;
  };
  bool is_ok;
} diplomat_result_box_ICU4XComposingNormalizer_ICU4XDataError;

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // diplomat_result_box_ICU4XComposingNormalizer_ICU4XDataError_D_H
