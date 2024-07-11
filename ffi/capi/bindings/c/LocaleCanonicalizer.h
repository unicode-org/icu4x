#ifndef LocaleCanonicalizer_H
#define LocaleCanonicalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataProvider.d.h"
#include "Error.d.h"
#include "Locale.d.h"
#include "TransformResult.d.h"

#include "LocaleCanonicalizer.d.h"






typedef struct ICU4XLocaleCanonicalizer_create_result {union {LocaleCanonicalizer* ok; Error err;}; bool is_ok;} ICU4XLocaleCanonicalizer_create_result;
ICU4XLocaleCanonicalizer_create_result ICU4XLocaleCanonicalizer_create(const DataProvider* provider);

typedef struct ICU4XLocaleCanonicalizer_create_extended_result {union {LocaleCanonicalizer* ok; Error err;}; bool is_ok;} ICU4XLocaleCanonicalizer_create_extended_result;
ICU4XLocaleCanonicalizer_create_extended_result ICU4XLocaleCanonicalizer_create_extended(const DataProvider* provider);

TransformResult ICU4XLocaleCanonicalizer_canonicalize(const LocaleCanonicalizer* self, Locale* locale);


void ICU4XLocaleCanonicalizer_destroy(LocaleCanonicalizer* self);





#endif // LocaleCanonicalizer_H
