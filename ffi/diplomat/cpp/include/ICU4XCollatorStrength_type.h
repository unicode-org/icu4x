#ifndef ICU4XCollatorStrength_type_H
#define ICU4XCollatorStrength_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef enum ICU4XCollatorStrength {
  ICU4XCollatorStrength_Auto = 0,
  ICU4XCollatorStrength_Primary = 1,
  ICU4XCollatorStrength_Secondary = 2,
  ICU4XCollatorStrength_Tertiary = 3,
  ICU4XCollatorStrength_Quaternary = 4,
  ICU4XCollatorStrength_Identical = 5,
} ICU4XCollatorStrength;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XCollatorStrength_type_H
