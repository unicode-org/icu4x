#ifndef ICU4XCalendar_H
#define ICU4XCalendar_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XAnyCalendarKind.d.h"
#include "ICU4XAnyCalendarKind.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XCalendar_ICU4XDataError.d.h"

#include "ICU4XCalendar.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XCalendar_ICU4XDataError ICU4XCalendar_create_for_locale(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XCalendar_ICU4XDataError ICU4XCalendar_create_for_kind(const ICU4XDataProvider* provider, ICU4XAnyCalendarKind kind);

ICU4XAnyCalendarKind ICU4XCalendar_kind(const ICU4XCalendar* self);

void ICU4XCalendar_destroy(ICU4XCalendar* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCalendar_H
