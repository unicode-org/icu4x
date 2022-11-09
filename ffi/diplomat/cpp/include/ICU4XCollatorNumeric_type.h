#ifndef ICU4XCollatorNumeric_type_H
#define ICU4XCollatorNumeric_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef enum ICU4XCollatorNumeric {
  ICU4XCollatorNumeric_Auto = 0,
  ICU4XCollatorNumeric_Off = 1,
  ICU4XCollatorNumeric_On = 2,
} ICU4XCollatorNumeric;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XCollatorNumeric_type_H
