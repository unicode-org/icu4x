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






typedef struct icu4x_RegionDisplayNames_create_mv1_result {union {RegionDisplayNames* ok; DataError err;}; bool is_ok;} icu4x_RegionDisplayNames_create_mv1_result;
icu4x_RegionDisplayNames_create_mv1_result icu4x_RegionDisplayNames_create_mv1(const DataProvider* provider, const Locale* locale);

typedef struct icu4x_RegionDisplayNames_of_mv1_result {union { LocaleParseError err;}; bool is_ok;} icu4x_RegionDisplayNames_of_mv1_result;
icu4x_RegionDisplayNames_of_mv1_result icu4x_RegionDisplayNames_of_mv1(const RegionDisplayNames* self, const char* region_data, size_t region_len, DiplomatWrite* write);


void icu4x_RegionDisplayNames_destroy_mv1(RegionDisplayNames* self);





#endif // RegionDisplayNames_H
