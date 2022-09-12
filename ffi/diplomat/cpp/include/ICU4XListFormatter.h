#ifndef ICU4XListFormatter_H
#define ICU4XListFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
namespace capi {
#endif

typedef struct ICU4XListFormatter ICU4XListFormatter;
#ifdef __cplusplus
} // namespace capi
#endif
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.h"
#include "ICU4XListStyle.h"
#include "diplomat_result_box_ICU4XListFormatter_ICU4XError.h"
#include "ICU4XList.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XListFormatter_ICU4XError ICU4XListFormatter_try_new_and(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XListStyle style);

diplomat_result_box_ICU4XListFormatter_ICU4XError ICU4XListFormatter_try_new_or(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XListStyle style);

diplomat_result_box_ICU4XListFormatter_ICU4XError ICU4XListFormatter_try_new_unit(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XListStyle style);

diplomat_result_void_ICU4XError ICU4XListFormatter_format(const ICU4XListFormatter* self, const ICU4XList* list, DiplomatWriteable* write);
void ICU4XListFormatter_destroy(ICU4XListFormatter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif
#endif
