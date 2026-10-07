#ifndef CompactDecimalFormatter_H
#define CompactDecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "Decimal.d.h"
#include "DecimalGroupingStrategy.d.h"
#include "Locale.d.h"

#include "CompactDecimalFormatter.d.h"






typedef struct icu4x_CompactDecimalFormatter_create_short_mv1_result {union {CompactDecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_CompactDecimalFormatter_create_short_mv1_result;
icu4x_CompactDecimalFormatter_create_short_mv1_result icu4x_CompactDecimalFormatter_create_short_mv1(const Locale* locale, DecimalGroupingStrategy_option grouping_strategy);

typedef struct icu4x_CompactDecimalFormatter_create_short_with_provider_mv1_result {union {CompactDecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_CompactDecimalFormatter_create_short_with_provider_mv1_result;
icu4x_CompactDecimalFormatter_create_short_with_provider_mv1_result icu4x_CompactDecimalFormatter_create_short_with_provider_mv1(const DataProvider* provider, const Locale* locale, DecimalGroupingStrategy_option grouping_strategy);

typedef struct icu4x_CompactDecimalFormatter_create_long_mv1_result {union {CompactDecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_CompactDecimalFormatter_create_long_mv1_result;
icu4x_CompactDecimalFormatter_create_long_mv1_result icu4x_CompactDecimalFormatter_create_long_mv1(const Locale* locale, DecimalGroupingStrategy_option grouping_strategy);

typedef struct icu4x_CompactDecimalFormatter_create_long_with_provider_mv1_result {union {CompactDecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_CompactDecimalFormatter_create_long_with_provider_mv1_result;
icu4x_CompactDecimalFormatter_create_long_with_provider_mv1_result icu4x_CompactDecimalFormatter_create_long_with_provider_mv1(const DataProvider* provider, const Locale* locale, DecimalGroupingStrategy_option grouping_strategy);

uint8_t icu4x_CompactDecimalFormatter_compact_exponent_for_magnitude_mv1(const CompactDecimalFormatter* self, int16_t magnitude);

void icu4x_CompactDecimalFormatter_format_mv1(const CompactDecimalFormatter* self, const Decimal* value, DiplomatWrite* write);

void icu4x_CompactDecimalFormatter_destroy_mv1(CompactDecimalFormatter* self);





#endif // CompactDecimalFormatter_H
