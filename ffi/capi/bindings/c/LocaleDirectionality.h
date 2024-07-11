#ifndef LocaleDirectionality_H
#define LocaleDirectionality_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "LocaleDirection.d.h"
#include "LocaleExpander.d.h"

#include "LocaleDirectionality.d.h"






typedef struct ICU4XLocaleDirectionality_create_result {union {LocaleDirectionality* ok; DataError err;}; bool is_ok;} ICU4XLocaleDirectionality_create_result;
ICU4XLocaleDirectionality_create_result ICU4XLocaleDirectionality_create(const DataProvider* provider);

typedef struct ICU4XLocaleDirectionality_create_with_expander_result {union {LocaleDirectionality* ok; DataError err;}; bool is_ok;} ICU4XLocaleDirectionality_create_with_expander_result;
ICU4XLocaleDirectionality_create_with_expander_result ICU4XLocaleDirectionality_create_with_expander(const DataProvider* provider, const LocaleExpander* expander);

LocaleDirection ICU4XLocaleDirectionality_get(const LocaleDirectionality* self, const Locale* locale);

bool ICU4XLocaleDirectionality_is_left_to_right(const LocaleDirectionality* self, const Locale* locale);

bool ICU4XLocaleDirectionality_is_right_to_left(const LocaleDirectionality* self, const Locale* locale);


void ICU4XLocaleDirectionality_destroy(LocaleDirectionality* self);





#endif // LocaleDirectionality_H
