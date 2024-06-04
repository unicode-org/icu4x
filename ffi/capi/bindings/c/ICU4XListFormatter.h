#ifndef ICU4XListFormatter_H
#define ICU4XListFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XListLength.d.h"
#include "ICU4XListLength.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XListFormatter_ICU4XError.d.h"

#include "ICU4XListFormatter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XListFormatter_ICU4XError ICU4XListFormatter_create_and_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XListLength length);

diplomat_result_box_ICU4XListFormatter_ICU4XError ICU4XListFormatter_create_or_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XListLength length);

diplomat_result_box_ICU4XListFormatter_ICU4XError ICU4XListFormatter_create_unit_with_length(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XListLength length);

void ICU4XListFormatter_format_valid_utf8(const ICU4XListFormatter* self, DiplomatStringsView* list_data, size_t list_len, DiplomatWrite* write);

void ICU4XListFormatter_format_utf8(const ICU4XListFormatter* self, DiplomatStringsView* list_data, size_t list_len, DiplomatWrite* write);

void ICU4XListFormatter_format_utf16(const ICU4XListFormatter* self, DiplomatStrings16View* list_data, size_t list_len, DiplomatWrite* write);

void ICU4XListFormatter_destroy(ICU4XListFormatter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XListFormatter_H
