#ifndef ICU4XTimeFormatter_H
#define ICU4XTimeFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XTimeFormatter ICU4XTimeFormatter;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XLocale.h"
#include "ICU4XDataProvider.h"
#include "ICU4XTimeLength.h"
#include "ICU4XHourCyclePreference.h"
#include "diplomat_result_box_ICU4XTimeFormatter_ICU4XError.h"
#include "ICU4XGregorianDateTime.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XTimeFormatter_ICU4XError ICU4XTimeFormatter_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XTimeLength length, ICU4XHourCyclePreference preferences);

diplomat_result_void_ICU4XError ICU4XTimeFormatter_format_gregorian_datetime(const ICU4XTimeFormatter* self, const ICU4XGregorianDateTime* value, DiplomatWriteable* write);
void ICU4XTimeFormatter_destroy(ICU4XTimeFormatter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
