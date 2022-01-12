#ifndef ICU4XCodePointSetDataResult_H
#define ICU4XCodePointSetDataResult_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XCodePointSetData ICU4XCodePointSetData;

typedef struct ICU4XCodePointSetDataResult {
    ICU4XCodePointSetData* data;
    bool success;
} ICU4XCodePointSetDataResult;

void ICU4XCodePointSetDataResult_destroy(ICU4XCodePointSetDataResult* self);

#ifdef __cplusplus
}
#endif
#endif
