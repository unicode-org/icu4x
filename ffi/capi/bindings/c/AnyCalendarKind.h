#ifndef AnyCalendarKind_H
#define AnyCalendarKind_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Locale.d.h"

#include "AnyCalendarKind.d.h"






typedef struct ICU4XAnyCalendarKind_get_for_locale_result {union {AnyCalendarKind ok; }; bool is_ok;} ICU4XAnyCalendarKind_get_for_locale_result;
ICU4XAnyCalendarKind_get_for_locale_result ICU4XAnyCalendarKind_get_for_locale(const Locale* locale);

typedef struct ICU4XAnyCalendarKind_get_for_bcp47_result {union {AnyCalendarKind ok; }; bool is_ok;} ICU4XAnyCalendarKind_get_for_bcp47_result;
ICU4XAnyCalendarKind_get_for_bcp47_result ICU4XAnyCalendarKind_get_for_bcp47(const char* s_data, size_t s_len);

void ICU4XAnyCalendarKind_bcp47(AnyCalendarKind self, DiplomatWrite* write);






#endif // AnyCalendarKind_H
