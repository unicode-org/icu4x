#ifndef ICU4XGregorianTimeFormatter_H
#define ICU4XGregorianTimeFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XGregorianTimeFormatter ICU4XGregorianTimeFormatter;
#include "ICU4XLocale.h"
#include "ICU4XDataProvider.h"
#include "ICU4XTimeLength.h"
#include "ICU4XHourCyclePreference.h"
#include "diplomat_result_box_ICU4XGregorianTimeFormatter_ICU4XError.h"
#include "ICU4XGregorianDateTime.h"
#include "diplomat_result_void_ICU4XError.h"

diplomat_result_box_ICU4XGregorianTimeFormatter_ICU4XError ICU4XGregorianTimeFormatter_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XTimeLength length, ICU4XHourCyclePreference preferences);

diplomat_result_void_ICU4XError ICU4XGregorianTimeFormatter_format_datetime(const ICU4XGregorianTimeFormatter* self, const ICU4XGregorianDateTime* value, DiplomatWriteable* write);
void ICU4XGregorianTimeFormatter_destroy(ICU4XGregorianTimeFormatter* self);

#ifdef __cplusplus
}
#endif
#endif
