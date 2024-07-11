#ifndef ListFormatter_H
#define ListFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "ListLength.d.h"
#include "Locale.d.h"

#include "ListFormatter.d.h"






typedef struct ICU4XListFormatter_create_and_with_length_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} ICU4XListFormatter_create_and_with_length_result;
ICU4XListFormatter_create_and_with_length_result ICU4XListFormatter_create_and_with_length(const DataProvider* provider, const Locale* locale, ListLength length);

typedef struct ICU4XListFormatter_create_or_with_length_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} ICU4XListFormatter_create_or_with_length_result;
ICU4XListFormatter_create_or_with_length_result ICU4XListFormatter_create_or_with_length(const DataProvider* provider, const Locale* locale, ListLength length);

typedef struct ICU4XListFormatter_create_unit_with_length_result {union {ListFormatter* ok; DataError err;}; bool is_ok;} ICU4XListFormatter_create_unit_with_length_result;
ICU4XListFormatter_create_unit_with_length_result ICU4XListFormatter_create_unit_with_length(const DataProvider* provider, const Locale* locale, ListLength length);

void ICU4XListFormatter_format_valid_utf8(const ListFormatter* self, DiplomatStringsView* list_data, size_t list_len, DiplomatWrite* write);

void ICU4XListFormatter_format_utf8(const ListFormatter* self, DiplomatStringsView* list_data, size_t list_len, DiplomatWrite* write);

void ICU4XListFormatter_format_utf16(const ListFormatter* self, DiplomatStrings16View* list_data, size_t list_len, DiplomatWrite* write);


void ICU4XListFormatter_destroy(ListFormatter* self);





#endif // ListFormatter_H
