#ifndef ICU4XLineBreakOptionsV1_H
#define ICU4XLineBreakOptionsV1_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XLineBreakRule.h"
#include "ICU4XWordBreakRule.h"
#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XLineBreakOptionsV1 {
    ICU4XLineBreakRule line_break_rule;
    ICU4XWordBreakRule word_break_rule;
    bool ja_zh;
} ICU4XLineBreakOptionsV1;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XLineBreakRule.h"
#include "ICU4XWordBreakRule.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XLineBreakOptionsV1_destroy(ICU4XLineBreakOptionsV1* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
