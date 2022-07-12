#ifndef ICU4XGregorianDateFormat_H
#define ICU4XGregorianDateFormat_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XGregorianDateFormat ICU4XGregorianDateFormat;
#include "ICU4XLocale.h"
#include "ICU4XDataProvider.h"
#include "ICU4XDateLength.h"
#include "diplomat_result_box_ICU4XGregorianDateFormat_ICU4XError.h"
#include "ICU4XGregorianDateTime.h"
#include "diplomat_result_void_ICU4XError.h"

diplomat_result_box_ICU4XGregorianDateFormat_ICU4XError ICU4XGregorianDateFormat_try_new(const ICU4XLocale* locale, const ICU4XDataProvider* provider, ICU4XDateLength length);

diplomat_result_void_ICU4XError ICU4XGregorianDateFormat_format_datetime(const ICU4XGregorianDateFormat* self, const ICU4XGregorianDateTime* value, DiplomatWriteable* write);
void ICU4XGregorianDateFormat_destroy(ICU4XGregorianDateFormat* self);

#ifdef __cplusplus
}
#endif
#endif
