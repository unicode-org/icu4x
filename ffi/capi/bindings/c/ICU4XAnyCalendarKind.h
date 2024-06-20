#ifndef ICU4XAnyCalendarKind_H
#define ICU4XAnyCalendarKind_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XLocale.d.h"

#include "ICU4XAnyCalendarKind.d.h"






struct ICU4XAnyCalendarKind_get_for_locale_result {union {ICU4XAnyCalendarKind ok; }; bool is_ok;};
struct ICU4XAnyCalendarKind_get_for_locale_result ICU4XAnyCalendarKind_get_for_locale(const ICU4XLocale* locale);

struct ICU4XAnyCalendarKind_get_for_bcp47_result {union {ICU4XAnyCalendarKind ok; }; bool is_ok;};
struct ICU4XAnyCalendarKind_get_for_bcp47_result ICU4XAnyCalendarKind_get_for_bcp47(const char* s_data, size_t s_len);

void ICU4XAnyCalendarKind_bcp47(ICU4XAnyCalendarKind self, DiplomatWrite* write);






#endif // ICU4XAnyCalendarKind_H
