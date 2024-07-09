#ifndef ICU4XCollatorResolvedOptionsV1_D_H
#define ICU4XCollatorResolvedOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XCollatorAlternateHandling.d.h"
#include "ICU4XCollatorBackwardSecondLevel.d.h"
#include "ICU4XCollatorCaseFirst.d.h"
#include "ICU4XCollatorCaseLevel.d.h"
#include "ICU4XCollatorMaxVariable.d.h"
#include "ICU4XCollatorNumeric.d.h"
#include "ICU4XCollatorStrength.d.h"




typedef struct ICU4XCollatorResolvedOptionsV1 {
  ICU4XCollatorStrength strength;
  ICU4XCollatorAlternateHandling alternate_handling;
  ICU4XCollatorCaseFirst case_first;
  ICU4XCollatorMaxVariable max_variable;
  ICU4XCollatorCaseLevel case_level;
  ICU4XCollatorNumeric numeric;
  ICU4XCollatorBackwardSecondLevel backward_second_level;
} ICU4XCollatorResolvedOptionsV1;





#endif // ICU4XCollatorResolvedOptionsV1_D_H
