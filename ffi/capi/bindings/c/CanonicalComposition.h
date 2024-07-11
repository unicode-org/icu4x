#ifndef CanonicalComposition_H
#define CanonicalComposition_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "CanonicalComposition.d.h"






typedef struct ICU4XCanonicalComposition_create_result {union {CanonicalComposition* ok; DataError err;}; bool is_ok;} ICU4XCanonicalComposition_create_result;
ICU4XCanonicalComposition_create_result ICU4XCanonicalComposition_create(const DataProvider* provider);

char32_t ICU4XCanonicalComposition_compose(const CanonicalComposition* self, char32_t starter, char32_t second);


void ICU4XCanonicalComposition_destroy(CanonicalComposition* self);





#endif // CanonicalComposition_H
