#ifndef ICU4XScriptExtensionsSet_H
#define ICU4XScriptExtensionsSet_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XScriptExtensionsSet_type.h"
#include "diplomat_result_uint16_t_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

bool ICU4XScriptExtensionsSet_contains(const ICU4XScriptExtensionsSet* self, uint16_t script);

size_t ICU4XScriptExtensionsSet_count(const ICU4XScriptExtensionsSet* self);

diplomat_result_uint16_t_void ICU4XScriptExtensionsSet_script_at(const ICU4XScriptExtensionsSet* self, size_t index);
void ICU4XScriptExtensionsSet_destroy(ICU4XScriptExtensionsSet* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XScriptExtensionsSet_H
