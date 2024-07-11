#ifndef LocaleExpander_H
#define LocaleExpander_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "Error.d.h"
#include "Locale.d.h"
#include "TransformResult.d.h"

#include "LocaleExpander.d.h"






typedef struct ICU4XLocaleExpander_create_result {union {LocaleExpander* ok; Error err;}; bool is_ok;} ICU4XLocaleExpander_create_result;
ICU4XLocaleExpander_create_result ICU4XLocaleExpander_create(const DataProvider* provider);

typedef struct ICU4XLocaleExpander_create_extended_result {union {LocaleExpander* ok; Error err;}; bool is_ok;} ICU4XLocaleExpander_create_extended_result;
ICU4XLocaleExpander_create_extended_result ICU4XLocaleExpander_create_extended(const DataProvider* provider);

TransformResult ICU4XLocaleExpander_maximize(const LocaleExpander* self, Locale* locale);

TransformResult ICU4XLocaleExpander_minimize(const LocaleExpander* self, Locale* locale);

TransformResult ICU4XLocaleExpander_minimize_favor_script(const LocaleExpander* self, Locale* locale);


void ICU4XLocaleExpander_destroy(LocaleExpander* self);





#endif // LocaleExpander_H
