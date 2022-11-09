#ifndef ICU4XScriptWithExtensions_H
#define ICU4XScriptWithExtensions_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XScriptWithExtensions_type.h"
#include "ICU4XDataProvider_type.h"
#include "diplomat_result_box_ICU4XScriptWithExtensions_ICU4XError.h"
#include "ICU4XScriptWithExtensionsBorrowed_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XScriptWithExtensions_ICU4XError ICU4XScriptWithExtensions_create(const ICU4XDataProvider* provider);

uint16_t ICU4XScriptWithExtensions_get_script_val(const ICU4XScriptWithExtensions* self, uint32_t code_point);

bool ICU4XScriptWithExtensions_has_script(const ICU4XScriptWithExtensions* self, uint32_t code_point, uint16_t script);

ICU4XScriptWithExtensionsBorrowed* ICU4XScriptWithExtensions_as_borrowed(const ICU4XScriptWithExtensions* self);
void ICU4XScriptWithExtensions_destroy(ICU4XScriptWithExtensions* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XScriptWithExtensions_H
