#ifndef ICU4XCollatorMaxVariable_D_H
#define ICU4XCollatorMaxVariable_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef enum ICU4XCollatorMaxVariable {
  ICU4XCollatorMaxVariable_Auto = 0,
  ICU4XCollatorMaxVariable_Space = 1,
  ICU4XCollatorMaxVariable_Punctuation = 2,
  ICU4XCollatorMaxVariable_Symbol = 3,
  ICU4XCollatorMaxVariable_Currency = 4,
} ICU4XCollatorMaxVariable;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCollatorMaxVariable_D_H
