#ifndef CollatorOptionsV1_D_H
#define CollatorOptionsV1_D_H

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




typedef struct CollatorOptionsV1 {
  CollatorStrength strength;
  CollatorAlternateHandling alternate_handling;
  CollatorCaseFirst case_first;
  CollatorMaxVariable max_variable;
  CollatorCaseLevel case_level;
  CollatorNumeric numeric;
  CollatorBackwardSecondLevel backward_second_level;
} CollatorOptionsV1;

typedef struct CollatorOptionsV1_option {union { CollatorOptionsV1 ok; }; bool is_ok; } CollatorOptionsV1_option;



#endif // CollatorOptionsV1_D_H
