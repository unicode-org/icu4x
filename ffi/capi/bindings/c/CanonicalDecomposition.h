#ifndef CanonicalDecomposition_H
#define CanonicalDecomposition_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Decomposed.d.h"

#include "CanonicalDecomposition.d.h"






typedef struct ICU4XCanonicalDecomposition_create_result {union {CanonicalDecomposition* ok; DataError err;}; bool is_ok;} ICU4XCanonicalDecomposition_create_result;
ICU4XCanonicalDecomposition_create_result ICU4XCanonicalDecomposition_create(const DataProvider* provider);

Decomposed ICU4XCanonicalDecomposition_decompose(const CanonicalDecomposition* self, char32_t c);


void ICU4XCanonicalDecomposition_destroy(CanonicalDecomposition* self);





#endif // CanonicalDecomposition_H
