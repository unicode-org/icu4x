#ifndef ICU4XGregorianDateTimeFormatter_H
#define ICU4XGregorianDateTimeFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XGregorianDateTimeFormatter ICU4XGregorianDateTimeFormatter;
#include "ICU4XLocale.h"
#include "ICU4XDataProvider.h"
#include "ICU4XDateLength.h"
#include "ICU4XTimeLength.h"
#include "ICU4XHourCyclePreference.h"
#include "diplomat_result_box_ICU4XGregorianDateTimeFormatter_ICU4XError.h"
#include "ICU4XGregorianDateTime.h"
#include "diplomat_result_void_ICU4XError.h"

diplomat_result_box_ICU4XGregorianDateTimeFormatter_ICU4XError ICU4XGregorianDateTimeFormatter_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XHourCyclePreference time_preferences);

diplomat_result_void_ICU4XError ICU4XGregorianDateTimeFormatter_format_datetime(const ICU4XGregorianDateTimeFormatter* self, const ICU4XGregorianDateTime* value, DiplomatWriteable* write);
void ICU4XGregorianDateTimeFormatter_destroy(ICU4XGregorianDateTimeFormatter* self);

#ifdef __cplusplus
}
#endif
#endif
