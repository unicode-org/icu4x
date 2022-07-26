#ifndef ICU4XCodePointMapData16Response_H
#define ICU4XCodePointMapData16Response_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

typedef struct ICU4XCodePointMapData16 ICU4XCodePointMapData16;
#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XCodePointMapData16Response {
    ICU4XCodePointMapData16* data;
    bool success;
} ICU4XCodePointMapData16Response;
#ifdef __cplusplus
} // namespace capi
#endif
typedef struct ICU4XCodePointMapData16 ICU4XCodePointMapData16;
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

void ICU4XCodePointMapData16Response_destroy(ICU4XCodePointMapData16Response* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
