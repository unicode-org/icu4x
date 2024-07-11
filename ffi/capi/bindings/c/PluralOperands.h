#ifndef PluralOperands_H
#define PluralOperands_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "FixedDecimal.d.h"
#include "FixedDecimalParseError.d.h"

#include "PluralOperands.d.h"






typedef struct ICU4XPluralOperands_create_from_string_result {union {PluralOperands* ok; FixedDecimalParseError err;}; bool is_ok;} ICU4XPluralOperands_create_from_string_result;
ICU4XPluralOperands_create_from_string_result ICU4XPluralOperands_create_from_string(const char* s_data, size_t s_len);

PluralOperands* ICU4XPluralOperands_create_from_fixed_decimal(const FixedDecimal* x);


void ICU4XPluralOperands_destroy(PluralOperands* self);





#endif // PluralOperands_H
