#ifndef ICU4XCreatePluralOperandsResult_H
#define ICU4XCreatePluralOperandsResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
#include "ICU4XPluralOperands.h"

typedef struct ICU4XCreatePluralOperandsResult {
    ICU4XPluralOperands operands;
    bool success;
} ICU4XCreatePluralOperandsResult;

void ICU4XCreatePluralOperandsResult_destroy(ICU4XCreatePluralOperandsResult* self);

#ifdef __cplusplus
}
#endif
#endif
