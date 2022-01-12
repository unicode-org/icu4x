#ifndef ICU4XLocaleCanonicalizer_H
#define ICU4XLocaleCanonicalizer_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XLocaleCanonicalizer ICU4XLocaleCanonicalizer;
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.h"
#include "ICU4XCanonicalizationResult.h"

ICU4XLocaleCanonicalizer* ICU4XLocaleCanonicalizer_create(const ICU4XDataProvider* provider);

ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer_canonicalize(const ICU4XLocaleCanonicalizer* self, ICU4XLocale* locale);

ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer_maximize(const ICU4XLocaleCanonicalizer* self, ICU4XLocale* locale);

ICU4XCanonicalizationResult ICU4XLocaleCanonicalizer_minimize(const ICU4XLocaleCanonicalizer* self, ICU4XLocale* locale);
void ICU4XLocaleCanonicalizer_destroy(ICU4XLocaleCanonicalizer* self);

#ifdef __cplusplus
}
#endif
#endif
