#ifndef ICU4XCollatorOptionsV1_type_H
#define ICU4XCollatorOptionsV1_type_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCollatorStrength_type.h"
#include "ICU4XCollatorAlternateHandling_type.h"
#include "ICU4XCollatorCaseFirst_type.h"
#include "ICU4XCollatorMaxVariable_type.h"
#include "ICU4XCollatorCaseLevel_type.h"
#include "ICU4XCollatorNumeric_type.h"
#include "ICU4XCollatorBackwardSecondLevel_type.h"
#ifdef __cplusplus
namespace capi {
#endif // __cplusplus

typedef struct ICU4XCollatorOptionsV1 {
    ICU4XCollatorStrength strength;
    ICU4XCollatorAlternateHandling alternate_handling;
    ICU4XCollatorCaseFirst case_first;
    ICU4XCollatorMaxVariable max_variable;
    ICU4XCollatorCaseLevel case_level;
    ICU4XCollatorNumeric numeric;
    ICU4XCollatorBackwardSecondLevel backward_second_level;
} ICU4XCollatorOptionsV1;
#ifdef __cplusplus
} // namespace capi
#endif // __cplusplus
#endif // ICU4XCollatorOptionsV1_type_H
