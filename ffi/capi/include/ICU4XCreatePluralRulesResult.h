#ifndef ICU4XCreatePluralRulesResult_H
#define ICU4XCreatePluralRulesResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XPluralRules ICU4XPluralRules;

typedef struct ICU4XCreatePluralRulesResult {
    ICU4XPluralRules* rules;
    bool success;
} ICU4XCreatePluralRulesResult;

void ICU4XCreatePluralRulesResult_destroy(ICU4XCreatePluralRulesResult* self);

#ifdef __cplusplus
}
#endif
#endif
