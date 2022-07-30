#ifndef ICU4XGregorianDateFormatter_H
#define ICU4XGregorianDateFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XGregorianDateFormatter ICU4XGregorianDateFormatter;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.h"
#include "ICU4XDateLength.h"
#include "diplomat_result_box_ICU4XGregorianDateFormatter_ICU4XError.h"
#include "ICU4XGregorianDateTime.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XGregorianDateFormatter_ICU4XError ICU4XGregorianDateFormatter_try_new(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XDateLength length);

diplomat_result_void_ICU4XError ICU4XGregorianDateFormatter_format_datetime(const ICU4XGregorianDateFormatter* self, const ICU4XGregorianDateTime* value, DiplomatWriteable* write);
void ICU4XGregorianDateFormatter_destroy(ICU4XGregorianDateFormatter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
