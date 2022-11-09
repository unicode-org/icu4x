#ifndef ICU4XLocaleExpander_H
#define ICU4XLocaleExpander_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLocaleExpander_type.h"
#include "ICU4XDataProvider_type.h"
#include "diplomat_result_box_ICU4XLocaleExpander_ICU4XError.h"
#include "ICU4XLocale_type.h"
#include "ICU4XTransformResult_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XLocaleExpander_ICU4XError ICU4XLocaleExpander_create(const ICU4XDataProvider* provider);

ICU4XTransformResult ICU4XLocaleExpander_maximize(const ICU4XLocaleExpander* self, ICU4XLocale* locale);

ICU4XTransformResult ICU4XLocaleExpander_minimize(const ICU4XLocaleExpander* self, ICU4XLocale* locale);
void ICU4XLocaleExpander_destroy(ICU4XLocaleExpander* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XLocaleExpander_H
