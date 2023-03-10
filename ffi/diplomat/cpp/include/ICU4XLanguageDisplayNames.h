#ifndef ICU4XLanguageDisplayNames_H
#define ICU4XLanguageDisplayNames_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XLanguageDisplayNames ICU4XLanguageDisplayNames;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XLanguageDisplayNames_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XLanguageDisplayNames_ICU4XError ICU4XLanguageDisplayNames_try_new_unstable(const ICU4XDataProvider* provider, const ICU4XLocale* locale);
void ICU4XLanguageDisplayNames_destroy(ICU4XLanguageDisplayNames* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
