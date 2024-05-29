#ifndef ICU4XRegionDisplayNames_H
#define ICU4XRegionDisplayNames_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XRegionDisplayNames_ICU4XDataError.d.h"
#include "diplomat_result_void_ICU4XLocaleParseError.d.h"

#include "ICU4XRegionDisplayNames.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XRegionDisplayNames_ICU4XDataError ICU4XRegionDisplayNames_create(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_void_ICU4XLocaleParseError ICU4XRegionDisplayNames_of(const ICU4XRegionDisplayNames* self, const char* region_data, size_t region_len, DiplomatWrite* write);

void ICU4XRegionDisplayNames_destroy(ICU4XRegionDisplayNames* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XRegionDisplayNames_H
