#ifndef ICU4XFixedDecimalFormatOptions_H
#define ICU4XFixedDecimalFormatOptions_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "ICU4XFixedDecimalGroupingStrategy.h"
#include "ICU4XFixedDecimalSignDisplay.h"

typedef struct ICU4XFixedDecimalFormatOptions {
    ICU4XFixedDecimalGroupingStrategy grouping_strategy;
    ICU4XFixedDecimalSignDisplay sign_display;
} ICU4XFixedDecimalFormatOptions;

ICU4XFixedDecimalFormatOptions ICU4XFixedDecimalFormatOptions_default();
void ICU4XFixedDecimalFormatOptions_destroy(ICU4XFixedDecimalFormatOptions* self);

#ifdef __cplusplus
}
#endif
#endif
