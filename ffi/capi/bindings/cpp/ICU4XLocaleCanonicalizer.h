#ifndef ICU4XLocaleCanonicalizer_H
#define ICU4XLocaleCanonicalizer_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "ICU4XTransformResult.d.h"
#include "ICU4XTransformResult.h"
#include "diplomat_result_box_ICU4XLocaleCanonicalizer_ICU4XError.d.h"

#include "ICU4XLocaleCanonicalizer.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XLocaleCanonicalizer_ICU4XError ICU4XLocaleCanonicalizer_create(const ICU4XDataProvider* provider);

diplomat_result_box_ICU4XLocaleCanonicalizer_ICU4XError ICU4XLocaleCanonicalizer_create_extended(const ICU4XDataProvider* provider);

ICU4XTransformResult ICU4XLocaleCanonicalizer_canonicalize(const ICU4XLocaleCanonicalizer* self, ICU4XLocale* locale);

void ICU4XLocaleCanonicalizer_destroy(ICU4XLocaleCanonicalizer* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XLocaleCanonicalizer_H
