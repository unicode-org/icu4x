#ifndef ICU4XLocaleExpander_H
#define ICU4XLocaleExpander_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataProvider.d.h"
#include "ICU4XError.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XTransformResult.d.h"

#include "ICU4XLocaleExpander.d.h"






struct ICU4XLocaleExpander_create_result {union {ICU4XLocaleExpander* ok; ICU4XError err;}; bool is_ok;};
struct ICU4XLocaleExpander_create_result ICU4XLocaleExpander_create(const ICU4XDataProvider* provider);

struct ICU4XLocaleExpander_create_extended_result {union {ICU4XLocaleExpander* ok; ICU4XError err;}; bool is_ok;};
struct ICU4XLocaleExpander_create_extended_result ICU4XLocaleExpander_create_extended(const ICU4XDataProvider* provider);

ICU4XTransformResult ICU4XLocaleExpander_maximize(const ICU4XLocaleExpander* self, ICU4XLocale* locale);

ICU4XTransformResult ICU4XLocaleExpander_minimize(const ICU4XLocaleExpander* self, ICU4XLocale* locale);

ICU4XTransformResult ICU4XLocaleExpander_minimize_favor_script(const ICU4XLocaleExpander* self, ICU4XLocale* locale);


void ICU4XLocaleExpander_destroy(ICU4XLocaleExpander* self);





#endif // ICU4XLocaleExpander_H
