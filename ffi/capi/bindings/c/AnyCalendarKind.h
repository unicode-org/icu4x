#ifndef AnyCalendarKind_H
#define AnyCalendarKind_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "Locale.d.h"

#include "AnyCalendarKind.d.h"






typedef struct icu4x_AnyCalendarKind_get_for_locale_mv1_result {union {AnyCalendarKind ok; }; bool is_ok;} icu4x_AnyCalendarKind_get_for_locale_mv1_result;
icu4x_AnyCalendarKind_get_for_locale_mv1_result icu4x_AnyCalendarKind_get_for_locale_mv1(const Locale* locale);

typedef struct icu4x_AnyCalendarKind_get_for_bcp47_mv1_result {union {AnyCalendarKind ok; }; bool is_ok;} icu4x_AnyCalendarKind_get_for_bcp47_mv1_result;
icu4x_AnyCalendarKind_get_for_bcp47_mv1_result icu4x_AnyCalendarKind_get_for_bcp47_mv1(const char* s_data, size_t s_len);

void icu4x_AnyCalendarKind_bcp47_mv1(AnyCalendarKind self, DiplomatWrite* write);






#endif // AnyCalendarKind_H
