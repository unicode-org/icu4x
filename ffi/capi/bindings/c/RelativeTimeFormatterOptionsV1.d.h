#ifndef RelativeTimeFormatterOptionsV1_D_H
#define RelativeTimeFormatterOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DecimalGroupingStrategy.d.h"
#include "RelativeTimeNumeric.d.h"




typedef struct RelativeTimeFormatterOptionsV1 {
  RelativeTimeNumeric_option numeric;
  DecimalGroupingStrategy_option grouping_strategy;
} RelativeTimeFormatterOptionsV1;

typedef struct RelativeTimeFormatterOptionsV1_option {union { RelativeTimeFormatterOptionsV1 ok; }; bool is_ok; } RelativeTimeFormatterOptionsV1_option;



#endif // RelativeTimeFormatterOptionsV1_D_H
