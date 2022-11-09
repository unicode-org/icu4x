#ifndef ICU4XCollatorOptionsV1_H
#define ICU4XCollatorOptionsV1_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCollatorOptionsV1_type.h"
#include "ICU4XCollatorStrength_type.h"
#include "ICU4XCollatorAlternateHandling_type.h"
#include "ICU4XCollatorCaseFirst_type.h"
#include "ICU4XCollatorMaxVariable_type.h"
#include "ICU4XCollatorCaseLevel_type.h"
#include "ICU4XCollatorNumeric_type.h"
#include "ICU4XCollatorBackwardSecondLevel_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XCollatorOptionsV1_destroy(ICU4XCollatorOptionsV1* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XCollatorOptionsV1_H
