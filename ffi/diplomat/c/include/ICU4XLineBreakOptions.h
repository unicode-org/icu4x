#ifndef ICU4XLineBreakOptions_H
#define ICU4XLineBreakOptions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "ICU4XLineBreakRule.h"
#include "ICU4XWordBreakRule.h"

typedef struct ICU4XLineBreakOptions {
    ICU4XLineBreakRule line_break_rule;
    ICU4XWordBreakRule word_break_rule;
    bool ja_zh;
} ICU4XLineBreakOptions;

void ICU4XLineBreakOptions_destroy(ICU4XLineBreakOptions* self);

#ifdef __cplusplus
}
#endif
#endif
