#ifndef ICU4XAnyCalendarKind_H
#define ICU4XAnyCalendarKind_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XAnyCalendarKind_type.h"
#include "ICU4XLocale_type.h"
#include "diplomat_result_ICU4XAnyCalendarKind_void.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_ICU4XAnyCalendarKind_void ICU4XAnyCalendarKind_get_for_locale(const ICU4XLocale* locale);

diplomat_result_ICU4XAnyCalendarKind_void ICU4XAnyCalendarKind_get_for_bcp47(const char* s_data, size_t s_len);

diplomat_result_void_ICU4XError ICU4XAnyCalendarKind_bcp47(ICU4XAnyCalendarKind self, DiplomatWriteable* write);
void ICU4XAnyCalendarKind_destroy(ICU4XAnyCalendarKind* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XAnyCalendarKind_H
