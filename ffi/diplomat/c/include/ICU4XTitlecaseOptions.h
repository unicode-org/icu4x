#ifndef ICU4XTitlecaseOptions_H
#define ICU4XTitlecaseOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XHeadAdjustment.h"
#include "ICU4XTailCasing.h"
#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XTitlecaseOptions {
    ICU4XHeadAdjustment head_adjustment;
    ICU4XTailCasing tail_casing;
} ICU4XTitlecaseOptions;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XHeadAdjustment.h"
#include "ICU4XTailCasing.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

ICU4XTitlecaseOptions ICU4XTitlecaseOptions_default_options();
void ICU4XTitlecaseOptions_destroy(ICU4XTitlecaseOptions* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
