#ifndef ICU4XPluralOperands_H
#define ICU4XPluralOperands_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XFixedDecimal.d.h"
#include "ICU4XFixedDecimal.h"
#include "diplomat_result_box_ICU4XPluralOperands_ICU4XPluralsParseError.d.h"

#include "ICU4XPluralOperands.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XPluralOperands_ICU4XPluralsParseError ICU4XPluralOperands_create_from_string(const char* s_data, size_t s_len);

ICU4XPluralOperands* ICU4XPluralOperands_create_from_fixed_decimal(const ICU4XFixedDecimal* x);

void ICU4XPluralOperands_destroy(ICU4XPluralOperands* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XPluralOperands_H
