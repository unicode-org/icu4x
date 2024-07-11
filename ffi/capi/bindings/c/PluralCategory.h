#ifndef PluralCategory_H
#define PluralCategory_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "PluralCategory.d.h"






typedef struct ICU4XPluralCategory_get_for_cldr_string_result {union {PluralCategory ok; }; bool is_ok;} ICU4XPluralCategory_get_for_cldr_string_result;
ICU4XPluralCategory_get_for_cldr_string_result ICU4XPluralCategory_get_for_cldr_string(const char* s_data, size_t s_len);






#endif // PluralCategory_H
