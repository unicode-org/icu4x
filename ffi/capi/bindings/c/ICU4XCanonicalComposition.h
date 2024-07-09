#ifndef ICU4XCanonicalComposition_H
#define ICU4XCanonicalComposition_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"

#include "ICU4XCanonicalComposition.d.h"






typedef struct ICU4XCanonicalComposition_create_result {union {ICU4XCanonicalComposition* ok; ICU4XDataError err;}; bool is_ok;} ICU4XCanonicalComposition_create_result;
ICU4XCanonicalComposition_create_result ICU4XCanonicalComposition_create(const ICU4XDataProvider* provider);

char32_t ICU4XCanonicalComposition_compose(const ICU4XCanonicalComposition* self, char32_t starter, char32_t second);


void ICU4XCanonicalComposition_destroy(ICU4XCanonicalComposition* self);





#endif // ICU4XCanonicalComposition_H
