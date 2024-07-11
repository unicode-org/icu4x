#ifndef RegionDisplayNames_H
#define RegionDisplayNames_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"
#include "LocaleParseError.d.h"

#include "RegionDisplayNames.d.h"






typedef struct ICU4XRegionDisplayNames_create_result {union {RegionDisplayNames* ok; DataError err;}; bool is_ok;} ICU4XRegionDisplayNames_create_result;
ICU4XRegionDisplayNames_create_result ICU4XRegionDisplayNames_create(const DataProvider* provider, const Locale* locale);

typedef struct ICU4XRegionDisplayNames_of_result {union { LocaleParseError err;}; bool is_ok;} ICU4XRegionDisplayNames_of_result;
ICU4XRegionDisplayNames_of_result ICU4XRegionDisplayNames_of(const RegionDisplayNames* self, const char* region_data, size_t region_len, DiplomatWrite* write);


void ICU4XRegionDisplayNames_destroy(RegionDisplayNames* self);





#endif // RegionDisplayNames_H
