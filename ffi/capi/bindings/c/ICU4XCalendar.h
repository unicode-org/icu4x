#ifndef ICU4XCalendar_H
#define ICU4XCalendar_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XAnyCalendarKind.d.h"
#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XLocale.d.h"

#include "ICU4XCalendar.d.h"






struct ICU4XCalendar_create_for_locale_result {union {ICU4XCalendar* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCalendar_create_for_locale_result ICU4XCalendar_create_for_locale(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

struct ICU4XCalendar_create_for_kind_result {union {ICU4XCalendar* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCalendar_create_for_kind_result ICU4XCalendar_create_for_kind(const ICU4XDataProvider* provider, ICU4XAnyCalendarKind kind);

ICU4XAnyCalendarKind ICU4XCalendar_kind(const ICU4XCalendar* self);


void ICU4XCalendar_destroy(ICU4XCalendar* self);





#endif // ICU4XCalendar_H
