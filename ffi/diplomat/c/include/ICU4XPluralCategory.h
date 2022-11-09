#ifndef ICU4XPluralCategory_H
#define ICU4XPluralCategory_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XPluralCategory_type.h"
#include "diplomat_result_ICU4XPluralCategory_void.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_ICU4XPluralCategory_void ICU4XPluralCategory_get_for_cldr_string(const char* s_data, size_t s_len);
void ICU4XPluralCategory_destroy(ICU4XPluralCategory* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XPluralCategory_H
