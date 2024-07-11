#ifndef TitlecaseOptionsV1_D_H
#define TitlecaseOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "LeadingAdjustment.d.h"
#include "TrailingCase.d.h"




typedef struct TitlecaseOptionsV1 {
  LeadingAdjustment leading_adjustment;
  TrailingCase trailing_case;
} TitlecaseOptionsV1;





#endif // TitlecaseOptionsV1_D_H
