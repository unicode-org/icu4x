#ifndef ICU4XDateFormatter_H
#define ICU4XDateFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XDate.d.h"
#include "ICU4XDate.h"
#include "ICU4XDateLength.d.h"
#include "ICU4XDateLength.h"
#include "ICU4XDateTime.d.h"
#include "ICU4XDateTime.h"
#include "ICU4XIsoDate.d.h"
#include "ICU4XIsoDate.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XIsoDateTime.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XDateFormatter_ICU4XError.d.h"
#include "diplomat_result_void_ICU4XError.d.h"

#include "ICU4XDateFormatter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XDateFormatter_ICU4XError ICU4XDateFormatter_create_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength date_length);

diplomat_result_void_ICU4XError ICU4XDateFormatter_format_date(const ICU4XDateFormatter* self, const ICU4XDate* value, DiplomatWrite* write);

diplomat_result_void_ICU4XError ICU4XDateFormatter_format_iso_date(const ICU4XDateFormatter* self, const ICU4XIsoDate* value, DiplomatWrite* write);

diplomat_result_void_ICU4XError ICU4XDateFormatter_format_datetime(const ICU4XDateFormatter* self, const ICU4XDateTime* value, DiplomatWrite* write);

diplomat_result_void_ICU4XError ICU4XDateFormatter_format_iso_datetime(const ICU4XDateFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);

void ICU4XDateFormatter_destroy(ICU4XDateFormatter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XDateFormatter_H
