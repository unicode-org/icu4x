#ifndef diplomat_result_box_ICU4XPluralOperands_ICU4XPluralsParseError_D_H
#define diplomat_result_box_ICU4XPluralOperands_ICU4XPluralsParseError_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XPluralOperands.d.h"
#include "ICU4XPluralsParseError.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct diplomat_result_box_ICU4XPluralOperands_ICU4XPluralsParseError {
  union {
    ICU4XPluralOperands* ok;
    ICU4XPluralsParseError err;
  };
  bool is_ok;
} diplomat_result_box_ICU4XPluralOperands_ICU4XPluralsParseError;

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // diplomat_result_box_ICU4XPluralOperands_ICU4XPluralsParseError_D_H
