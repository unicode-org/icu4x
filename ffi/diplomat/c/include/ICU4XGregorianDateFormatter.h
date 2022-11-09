#ifndef ICU4XGregorianDateFormatter_H
#define ICU4XGregorianDateFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XGregorianDateFormatter_type.h"
#include "ICU4XDataProvider_type.h"
#include "ICU4XLocale_type.h"
#include "ICU4XDateLength_type.h"
#include "diplomat_result_box_ICU4XGregorianDateFormatter_ICU4XError.h"
#include "ICU4XIsoDate_type.h"
#include "diplomat_result_void_ICU4XError.h"
#include "ICU4XIsoDateTime_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XGregorianDateFormatter_ICU4XError ICU4XGregorianDateFormatter_create_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength length);

diplomat_result_void_ICU4XError ICU4XGregorianDateFormatter_format_iso_date(const ICU4XGregorianDateFormatter* self, const ICU4XIsoDate* value, DiplomatWriteable* write);

diplomat_result_void_ICU4XError ICU4XGregorianDateFormatter_format_iso_datetime(const ICU4XGregorianDateFormatter* self, const ICU4XIsoDateTime* value, DiplomatWriteable* write);
void ICU4XGregorianDateFormatter_destroy(ICU4XGregorianDateFormatter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XGregorianDateFormatter_H
