#ifndef ICU4XGregorianDateTimeFormatter_H
#define ICU4XGregorianDateTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XDateLength.d.h"
#include "ICU4XDateLength.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XIsoDateTime.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "ICU4XTimeLength.d.h"
#include "ICU4XTimeLength.h"
#include "diplomat_result_box_ICU4XGregorianDateTimeFormatter_ICU4XError.d.h"

#include "ICU4XGregorianDateTimeFormatter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XGregorianDateTimeFormatter_ICU4XError ICU4XGregorianDateTimeFormatter_create_with_lengths(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

void ICU4XGregorianDateTimeFormatter_format_iso_datetime(const ICU4XGregorianDateTimeFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);

void ICU4XGregorianDateTimeFormatter_destroy(ICU4XGregorianDateTimeFormatter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XGregorianDateTimeFormatter_H
