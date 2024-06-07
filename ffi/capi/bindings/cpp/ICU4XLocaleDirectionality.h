#ifndef ICU4XLocaleDirectionality_H
#define ICU4XLocaleDirectionality_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "ICU4XLocaleDirection.d.h"
#include "ICU4XLocaleDirection.h"
#include "ICU4XLocaleExpander.d.h"
#include "ICU4XLocaleExpander.h"
#include "diplomat_result_box_ICU4XLocaleDirectionality_ICU4XError.d.h"

#include "ICU4XLocaleDirectionality.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XLocaleDirectionality_ICU4XError ICU4XLocaleDirectionality_create(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XLocaleDirectionality_ICU4XError ICU4XLocaleDirectionality_create_with_expander(const ICU4XDataProvider* provider, const ICU4XLocaleExpander* expander);

ICU4XLocaleDirection ICU4XLocaleDirectionality_get(const ICU4XLocaleDirectionality* self, const ICU4XLocale* locale);

bool ICU4XLocaleDirectionality_is_left_to_right(const ICU4XLocaleDirectionality* self, const ICU4XLocale* locale);

bool ICU4XLocaleDirectionality_is_right_to_left(const ICU4XLocaleDirectionality* self, const ICU4XLocale* locale);

void ICU4XLocaleDirectionality_destroy(ICU4XLocaleDirectionality* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocaleDirectionality_H
