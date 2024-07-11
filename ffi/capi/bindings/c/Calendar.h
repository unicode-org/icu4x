#ifndef Calendar_H
#define Calendar_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "AnyCalendarKind.d.h"
#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"

#include "Calendar.d.h"






typedef struct ICU4XCalendar_create_for_locale_result {union {Calendar* ok; DataError err;}; bool is_ok;} ICU4XCalendar_create_for_locale_result;
ICU4XCalendar_create_for_locale_result ICU4XCalendar_create_for_locale(const DataProvider* provider, const Locale* locale);

typedef struct ICU4XCalendar_create_for_kind_result {union {Calendar* ok; DataError err;}; bool is_ok;} ICU4XCalendar_create_for_kind_result;
ICU4XCalendar_create_for_kind_result ICU4XCalendar_create_for_kind(const DataProvider* provider, AnyCalendarKind kind);

AnyCalendarKind ICU4XCalendar_kind(const Calendar* self);


void ICU4XCalendar_destroy(Calendar* self);





#endif // Calendar_H
