#ifndef CanonicalCombiningClassMap_H
#define CanonicalCombiningClassMap_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"

#include "CanonicalCombiningClassMap.d.h"






typedef struct ICU4XCanonicalCombiningClassMap_create_result {union {CanonicalCombiningClassMap* ok; DataError err;}; bool is_ok;} ICU4XCanonicalCombiningClassMap_create_result;
ICU4XCanonicalCombiningClassMap_create_result ICU4XCanonicalCombiningClassMap_create(const DataProvider* provider);

uint8_t ICU4XCanonicalCombiningClassMap_get(const CanonicalCombiningClassMap* self, char32_t ch);

uint8_t ICU4XCanonicalCombiningClassMap_get32(const CanonicalCombiningClassMap* self, uint32_t ch);


void ICU4XCanonicalCombiningClassMap_destroy(CanonicalCombiningClassMap* self);





#endif // CanonicalCombiningClassMap_H
