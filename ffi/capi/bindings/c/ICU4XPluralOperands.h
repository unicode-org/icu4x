#ifndef ICU4XPluralOperands_H
#define ICU4XPluralOperands_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XFixedDecimal.d.h"
#include "ICU4XFixedDecimalParseError.d.h"

#include "ICU4XPluralOperands.d.h"






typedef struct ICU4XPluralOperands_create_from_string_result {union {ICU4XPluralOperands* ok; ICU4XFixedDecimalParseError err;}; bool is_ok;} ICU4XPluralOperands_create_from_string_result;
ICU4XPluralOperands_create_from_string_result ICU4XPluralOperands_create_from_string(const char* s_data, size_t s_len);

ICU4XPluralOperands* ICU4XPluralOperands_create_from_fixed_decimal(const ICU4XFixedDecimal* x);


void ICU4XPluralOperands_destroy(ICU4XPluralOperands* self);





#endif // ICU4XPluralOperands_H
