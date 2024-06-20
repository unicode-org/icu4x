#ifndef ICU4XLocaleCanonicalizer_H
#define ICU4XLocaleCanonicalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataProvider.d.h"
#include "ICU4XError.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XTransformResult.d.h"

#include "ICU4XLocaleCanonicalizer.d.h"






struct ICU4XLocaleCanonicalizer_create_result {union {ICU4XLocaleCanonicalizer* ok; ICU4XError err;}; bool is_ok;};
struct ICU4XLocaleCanonicalizer_create_result ICU4XLocaleCanonicalizer_create(const ICU4XDataProvider* provider);

struct ICU4XLocaleCanonicalizer_create_extended_result {union {ICU4XLocaleCanonicalizer* ok; ICU4XError err;}; bool is_ok;};
struct ICU4XLocaleCanonicalizer_create_extended_result ICU4XLocaleCanonicalizer_create_extended(const ICU4XDataProvider* provider);

ICU4XTransformResult ICU4XLocaleCanonicalizer_canonicalize(const ICU4XLocaleCanonicalizer* self, ICU4XLocale* locale);


void ICU4XLocaleCanonicalizer_destroy(ICU4XLocaleCanonicalizer* self);





#endif // ICU4XLocaleCanonicalizer_H
