#ifndef ICU4XPluralCategory_H
#define ICU4XPluralCategory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "diplomat_result_ICU4XPluralCategory_void.d.h"

#include "ICU4XPluralCategory.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_ICU4XPluralCategory_void ICU4XPluralCategory_get_for_cldr_string(const char* s_data, size_t s_len);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XPluralCategory_H
