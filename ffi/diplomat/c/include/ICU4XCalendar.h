#ifndef ICU4XCalendar_H
#define ICU4XCalendar_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XCalendar_type.h"
#include "ICU4XDataProvider_type.h"
#include "ICU4XLocale_type.h"
#include "diplomat_result_box_ICU4XCalendar_ICU4XError.h"
#include "ICU4XAnyCalendarKind_type.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XCalendar_ICU4XError ICU4XCalendar_create_for_locale(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

diplomat_result_box_ICU4XCalendar_ICU4XError ICU4XCalendar_create_for_kind(const ICU4XDataProvider* provider, ICU4XAnyCalendarKind kind);

ICU4XAnyCalendarKind ICU4XCalendar_kind(const ICU4XCalendar* self);
void ICU4XCalendar_destroy(ICU4XCalendar* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XCalendar_H
