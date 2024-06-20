#ifndef ICU4XLocaleDirectionality_H
#define ICU4XLocaleDirectionality_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocaleDirection.d.h"
#include "ICU4XLocaleExpander.d.h"

#include "ICU4XLocaleDirectionality.d.h"






struct ICU4XLocaleDirectionality_create_result {union {ICU4XLocaleDirectionality* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XLocaleDirectionality_create_result ICU4XLocaleDirectionality_create(const ICU4XDataProvider* provider);

struct ICU4XLocaleDirectionality_create_with_expander_result {union {ICU4XLocaleDirectionality* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XLocaleDirectionality_create_with_expander_result ICU4XLocaleDirectionality_create_with_expander(const ICU4XDataProvider* provider, const ICU4XLocaleExpander* expander);

ICU4XLocaleDirection ICU4XLocaleDirectionality_get(const ICU4XLocaleDirectionality* self, const ICU4XLocale* locale);

bool ICU4XLocaleDirectionality_is_left_to_right(const ICU4XLocaleDirectionality* self, const ICU4XLocale* locale);

bool ICU4XLocaleDirectionality_is_right_to_left(const ICU4XLocaleDirectionality* self, const ICU4XLocale* locale);


void ICU4XLocaleDirectionality_destroy(ICU4XLocaleDirectionality* self);





#endif // ICU4XLocaleDirectionality_H
