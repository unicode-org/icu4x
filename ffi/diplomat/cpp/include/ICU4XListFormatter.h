#ifndef ICU4XListFormatter_H
#define ICU4XListFormatter_H
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XListFormatter_type.h"
#include "ICU4XDataProvider_type.h"
#include "ICU4XLocale_type.h"
#include "ICU4XListLength_type.h"
#include "diplomat_result_box_ICU4XListFormatter_ICU4XError.h"
#include "ICU4XList_type.h"
#include "diplomat_result_void_ICU4XError.h"
#ifdef __cplusplus
namespace capi {
extern "C" {
#endif

diplomat_result_box_ICU4XListFormatter_ICU4XError ICU4XListFormatter_create_and_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XListLength length);

diplomat_result_box_ICU4XListFormatter_ICU4XError ICU4XListFormatter_create_or_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XListLength length);

diplomat_result_box_ICU4XListFormatter_ICU4XError ICU4XListFormatter_create_unit_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XListLength length);

diplomat_result_void_ICU4XError ICU4XListFormatter_format(const ICU4XListFormatter* self, const ICU4XList* list, DiplomatWriteable* write);
void ICU4XListFormatter_destroy(ICU4XListFormatter* self);

#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus
#endif // ICU4XListFormatter_H
