#ifndef ICU4XFixedDecimalFormatOptions_H
#define ICU4XFixedDecimalFormatOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "ICU4XFixedDecimalGroupingStrategy.h"

typedef struct ICU4XFixedDecimalFormatOptions {
    ICU4XFixedDecimalGroupingStrategy grouping_strategy;
    int8_t work_around_diplomat_issue_173_do_not_use_this_field;
} ICU4XFixedDecimalFormatOptions;

ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions_default();
void ICU4XFixedDecimalFormatOptions_destroy(ICU4XFixedDecimalFormatOptions* self);

#ifdef __cplusplus
}
#endif
#endif
