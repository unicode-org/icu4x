#ifndef LocaleNamesUnstable_H
#define LocaleNamesUnstable_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Locale.d.h"

#include "LocaleNamesUnstable.d.h"






typedef struct icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1_result {union { DataError err;}; bool is_ok;} icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1_result;
icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1_result icu4x_LocaleNamesUnstable_for_region_with_compiled_data_light_mv1(const Locale* locale, DiplomatStringView region, DiplomatWrite* write);

typedef struct icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1_result {union { DataError err;}; bool is_ok;} icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1_result;
icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1_result icu4x_LocaleNamesUnstable_for_region_with_provider_light_mv1(const DataProvider* provider, const Locale* locale, DiplomatStringView region, DiplomatWrite* write);

void icu4x_LocaleNamesUnstable_destroy_mv1(LocaleNamesUnstable* self);





#endif // LocaleNamesUnstable_H
