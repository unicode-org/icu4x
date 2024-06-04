#ifndef ICU4XAnyCalendarKind_H
#define ICU4XAnyCalendarKind_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_ICU4XAnyCalendarKind_void.d.h"

#include "ICU4XAnyCalendarKind.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_ICU4XAnyCalendarKind_void ICU4XAnyCalendarKind_get_for_locale(const ICU4XLocale* locale);

diplomat_result_ICU4XAnyCalendarKind_void ICU4XAnyCalendarKind_get_for_bcp47(const char* s_data, size_t s_len);

void ICU4XAnyCalendarKind_bcp47(ICU4XAnyCalendarKind self, DiplomatWrite* write);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XAnyCalendarKind_H
