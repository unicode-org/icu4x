#ifndef ICU4XTitlecaseOptionsV1_D_H
#define ICU4XTitlecaseOptionsV1_D_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLeadingAdjustment.d.h"
#include "ICU4XTrailingCase.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


typedef struct ICU4XTitlecaseOptionsV1 {
  ICU4XLeadingAdjustment leading_adjustment;
  ICU4XTrailingCase trailing_case;
} ICU4XTitlecaseOptionsV1;


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XTitlecaseOptionsV1_D_H
