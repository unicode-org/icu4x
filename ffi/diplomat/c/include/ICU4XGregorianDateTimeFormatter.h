#ifndef ICU4XGregorianDateTimeFormatter_H
#define ICU4XGregorianDateTimeFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XGregorianDateTimeFormatter_type.h"
#include "ICU4XDataProvider_type.h"
#include "ICU4XLocale_type.h"
#include "ICU4XDateLength_type.h"
#include "ICU4XTimeLength_type.h"
#include "diplomat_result_box_ICU4XGregorianDateTimeFormatter_ICU4XError.h"
#include "ICU4XIsoDateTime_type.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XGregorianDateTimeFormatter_ICU4XError ICU4XGregorianDateTimeFormatter_create_with_lengths(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length, ICU4XTimeLength time_length);

diplomat_result_void_ICU4XError ICU4XGregorianDateTimeFormatter_format_iso_datetime(const ICU4XGregorianDateTimeFormatter* self, const ICU4XIsoDateTime* value, DiplomatWriteable* write);
void ICU4XGregorianDateTimeFormatter_destroy(ICU4XGregorianDateTimeFormatter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XGregorianDateTimeFormatter_H
