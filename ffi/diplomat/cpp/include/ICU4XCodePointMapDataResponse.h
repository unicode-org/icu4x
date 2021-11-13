#ifndef ICU4XCodePointMapDataResponse_H
#define ICU4XCodePointMapDataResponse_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif
typedef struct ICU4XCodePointMapData16 ICU4XCodePointMapData16;

typedef struct ICU4XCodePointMapDataResponse {
    ICU4XCodePointMapData16* data;
    bool success;
} ICU4XCodePointMapDataResponse;

void ICU4XCodePointMapDataResponse_destroy(ICU4XCodePointMapDataResponse* self);

#ifdef __cplusplus
}
#endif
#endif
