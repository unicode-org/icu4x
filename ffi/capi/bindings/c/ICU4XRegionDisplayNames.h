#ifndef ICU4XRegionDisplayNames_H
#define ICU4XRegionDisplayNames_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocaleParseError.d.h"

#include "ICU4XRegionDisplayNames.d.h"






typedef struct ICU4XRegionDisplayNames_create_result {union {ICU4XRegionDisplayNames* ok; ICU4XDataError err;}; bool is_ok;} ICU4XRegionDisplayNames_create_result;
ICU4XRegionDisplayNames_create_result ICU4XRegionDisplayNames_create(const ICU4XDataProvider* provider, const ICU4XLocale* locale);

typedef struct ICU4XRegionDisplayNames_of_result {union { ICU4XLocaleParseError err;}; bool is_ok;} ICU4XRegionDisplayNames_of_result;
ICU4XRegionDisplayNames_of_result ICU4XRegionDisplayNames_of(const ICU4XRegionDisplayNames* self, const char* region_data, size_t region_len, DiplomatWrite* write);


void ICU4XRegionDisplayNames_destroy(ICU4XRegionDisplayNames* self);





#endif // ICU4XRegionDisplayNames_H
