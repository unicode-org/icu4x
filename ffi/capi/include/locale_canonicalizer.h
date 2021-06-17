// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#ifndef ICU4X_LOCALE_CANONICALIZER_H
#define ICU4X_LOCALE_CANONICALIZER_H

#include "provider.h"
#include "locale.h"

typedef enum {
    ICU4XCanonicalizationResult_Modified,
    ICU4XCanonicalizationResult_Unmodified,
} ICU4XCanonicalizationResult;

// opaque
typedef struct ICU4XLocaleCanonicalizer ICU4XLocaleCanonicalizer;

ICU4XLocaleCanonicalizer* icu4x_localecanonicalizer_create(const ICU4XDataProvider* provider);

ICU4XCanonicalizationResult
icu4x_localecanonicalizer_canonicalize(const ICU4XLocaleCanonicalizer* lc, ICU4XLocale* locale);

ICU4XCanonicalizationResult
icu4x_localecanonicalizer_maximize(const ICU4XLocaleCanonicalizer* lc, ICU4XLocale* locale);

ICU4XCanonicalizationResult
icu4x_localecanonicalizer_minimize(const ICU4XLocaleCanonicalizer* lc, ICU4XLocale* locale);

void icu4x_localecanonicalizer_destroy(ICU4XLocaleCanonicalizer* lc);

#endif // ICU4X_LOCALE_CANONICALIZER_H
