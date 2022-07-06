#ifndef ICU4XGregorianDateTimeFormat_H
#define ICU4XGregorianDateTimeFormat_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XGregorianDateTimeFormat ICU4XGregorianDateTimeFormat;
#include "ICU4XLocale.h"
#include "ICU4XDataProvider.h"
#include "ICU4XDateLength.h"
#include "ICU4XTimeLength.h"
#include "ICU4XHourCyclePreference.h"
#include "diplomat_result_box_ICU4XGregorianDateTimeFormat_ICU4XError.h"
#include "ICU4XGregorianDateTime.h"
#include "diplomat_result_void_ICU4XError.h"

diplomat_result_box_ICU4XGregorianDateTimeFormat_ICU4XError ICU4XGregorianDateTimeFormat_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XDateLength date_length, ICU4XTimeLength time_length, ICU4XHourCyclePreference time_preferences);

diplomat_result_void_ICU4XError ICU4XGregorianDateTimeFormat_format_to_write(const ICU4XGregorianDateTimeFormat* self, const ICU4XGregorianDateTime* value, DiplomatWriteable* write);
void ICU4XGregorianDateTimeFormat_destroy(ICU4XGregorianDateTimeFormat* self);

#ifdef __cplusplus
}
#endif
#endif
