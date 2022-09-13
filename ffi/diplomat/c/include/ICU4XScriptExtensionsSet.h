#ifndef ICU4XScriptExtensionsSet_H
#define ICU4XScriptExtensionsSet_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XScriptExtensionsSet ICU4XScriptExtensionsSet;
#ifdef __cplusplus
} // namespace capi
#endif
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

bool ICU4XScriptExtensionsSet_contains(const ICU4XScriptExtensionsSet* self, uint16_t script);
void ICU4XScriptExtensionsSet_destroy(ICU4XScriptExtensionsSet* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
