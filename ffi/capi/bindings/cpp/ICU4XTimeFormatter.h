#ifndef ICU4XTimeFormatter_H
#define ICU4XTimeFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XDateTime.d.h"
#include "ICU4XDateTime.h"
#include "ICU4XIsoDateTime.d.h"
#include "ICU4XIsoDateTime.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "ICU4XTime.d.h"
#include "ICU4XTime.h"
#include "ICU4XTimeLength.d.h"
#include "ICU4XTimeLength.h"
#include "diplomat_result_box_ICU4XTimeFormatter_ICU4XError.d.h"

#include "ICU4XTimeFormatter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XTimeFormatter_ICU4XError ICU4XTimeFormatter_create_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XTimeLength length);

void ICU4XTimeFormatter_format_time(const ICU4XTimeFormatter* self, const ICU4XTime* value, DiplomatWrite* write);

void ICU4XTimeFormatter_format_datetime(const ICU4XTimeFormatter* self, const ICU4XDateTime* value, DiplomatWrite* write);

void ICU4XTimeFormatter_format_iso_datetime(const ICU4XTimeFormatter* self, const ICU4XIsoDateTime* value, DiplomatWrite* write);

void ICU4XTimeFormatter_destroy(ICU4XTimeFormatter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XTimeFormatter_H
