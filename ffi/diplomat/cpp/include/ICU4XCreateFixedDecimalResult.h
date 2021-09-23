#ifndef ICU4XCreateFixedDecimalResult_H
#define ICU4XCreateFixedDecimalResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XFixedDecimal ICU4XFixedDecimal;

typedef struct ICU4XCreateFixedDecimalResult {
    ICU4XFixedDecimal* fd;
    bool success;
} ICU4XCreateFixedDecimalResult;

void ICU4XCreateFixedDecimalResult_destroy(ICU4XCreateFixedDecimalResult* self);

#ifdef __cplusplus
}
#endif
#endif
