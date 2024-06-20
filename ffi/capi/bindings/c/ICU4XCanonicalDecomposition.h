#ifndef ICU4XCanonicalDecomposition_H
#define ICU4XCanonicalDecomposition_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDecomposed.d.h"

#include "ICU4XCanonicalDecomposition.d.h"






struct ICU4XCanonicalDecomposition_create_result {union {ICU4XCanonicalDecomposition* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XCanonicalDecomposition_create_result ICU4XCanonicalDecomposition_create(const ICU4XDataProvider* provider);

ICU4XDecomposed ICU4XCanonicalDecomposition_decompose(const ICU4XCanonicalDecomposition* self, char32_t c);


void ICU4XCanonicalDecomposition_destroy(ICU4XCanonicalDecomposition* self);





#endif // ICU4XCanonicalDecomposition_H
