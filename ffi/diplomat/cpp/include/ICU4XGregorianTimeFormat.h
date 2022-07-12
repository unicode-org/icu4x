#ifndef ICU4XGregorianTimeFormat_H
#define ICU4XGregorianTimeFormat_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XGregorianTimeFormat ICU4XGregorianTimeFormat;
#include "ICU4XLocale.h"
#include "ICU4XDataProvider.h"
#include "ICU4XTimeLength.h"
#include "ICU4XHourCyclePreference.h"
#include "diplomat_result_box_ICU4XGregorianTimeFormat_ICU4XError.h"
#include "ICU4XGregorianDateTime.h"
#include "diplomat_result_void_ICU4XError.h"

diplomat_result_box_ICU4XGregorianTimeFormat_ICU4XError ICU4XGregorianTimeFormat_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XTimeLength length, ICU4XHourCyclePreference preferences);

diplomat_result_void_ICU4XError ICU4XGregorianTimeFormat_format_datetime(const ICU4XGregorianTimeFormat* self, const ICU4XGregorianDateTime* value, DiplomatWriteable* write);
void ICU4XGregorianTimeFormat_destroy(ICU4XGregorianTimeFormat* self);

#ifdef __cplusplus
}
#endif
#endif
