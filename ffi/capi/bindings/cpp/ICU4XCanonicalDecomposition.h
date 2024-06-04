#ifndef ICU4XCanonicalDecomposition_H
#define ICU4XCanonicalDecomposition_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XDecomposed.d.h"
#include "ICU4XDecomposed.h"
#include "diplomat_result_box_ICU4XCanonicalDecomposition_ICU4XError.d.h"

#include "ICU4XCanonicalDecomposition.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XCanonicalDecomposition_ICU4XError ICU4XCanonicalDecomposition_create(const ICU4XDataProvider* provider);

ICU4XDecomposed ICU4XCanonicalDecomposition_decompose(const ICU4XCanonicalDecomposition* self, char32_t c);

void ICU4XCanonicalDecomposition_destroy(ICU4XCanonicalDecomposition* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XCanonicalDecomposition_H
