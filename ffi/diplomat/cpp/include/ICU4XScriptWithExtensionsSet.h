#ifndef ICU4XScriptWithExtensionsSet_H
#define ICU4XScriptWithExtensionsSet_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XScriptWithExtensionsSet ICU4XScriptWithExtensionsSet;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "diplomat_result_box_ICU4XScriptWithExtensionsSet_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XScriptWithExtensionsSet_ICU4XError ICU4XScriptWithExtensionsSet_load(const ICU4XDataProvider* provider);

uint16_t ICU4XScriptWithExtensionsSet_get_script_val(const ICU4XScriptWithExtensionsSet* self, uint32_t code_point);

bool ICU4XScriptWithExtensionsSet_has_script(const ICU4XScriptWithExtensionsSet* self, uint32_t code_point, uint16_t script);
void ICU4XScriptWithExtensionsSet_destroy(ICU4XScriptWithExtensionsSet* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
