#ifndef UnicodeSetData_H
#define UnicodeSetData_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "UnicodeSetData.d.h"






bool icu4x_UnicodeSetData_contains_mv1(const UnicodeSetData* self, DiplomatStringView s);

bool icu4x_UnicodeSetData_contains_char_mv1(const UnicodeSetData* self, char32_t cp);

typedef struct icu4x_UnicodeSetData_load_basic_emoji_mv1_result {union {UnicodeSetData* ok; DataError err;}; bool is_ok;} icu4x_UnicodeSetData_load_basic_emoji_mv1_result;
icu4x_UnicodeSetData_load_basic_emoji_mv1_result icu4x_UnicodeSetData_load_basic_emoji_mv1(const DataProvider* provider);


void icu4x_UnicodeSetData_destroy_mv1(UnicodeSetData* self);





#endif // UnicodeSetData_H
