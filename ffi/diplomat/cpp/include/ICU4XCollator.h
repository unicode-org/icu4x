#ifndef ICU4XCollator_H
#define ICU4XCollator_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XCollator ICU4XCollator;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.h"
#include "ICU4XCollatorOptions.h"
#include "diplomat_result_box_ICU4XCollator_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XCollator_ICU4XError ICU4XCollator_try_new(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XCollatorOptions options);
void ICU4XCollator_destroy(ICU4XCollator* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
