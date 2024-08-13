#ifndef CollatorResolvedOptionsV1_D_H
#define CollatorResolvedOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "CollatorAlternateHandling.d.h"
#include "CollatorBackwardSecondLevel.d.h"
#include "CollatorCaseFirst.d.h"
#include "CollatorCaseLevel.d.h"
#include "CollatorMaxVariable.d.h"
#include "CollatorNumeric.d.h"
#include "CollatorStrength.d.h"




typedef struct CollatorResolvedOptionsV1 {
  CollatorStrength strength;
  CollatorAlternateHandling alternate_handling;
  CollatorCaseFirst case_first;
  CollatorMaxVariable max_variable;
  CollatorCaseLevel case_level;
  CollatorNumeric numeric;
  CollatorBackwardSecondLevel backward_second_level;
} CollatorResolvedOptionsV1;





#endif // CollatorResolvedOptionsV1_D_H
