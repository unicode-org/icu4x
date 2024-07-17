#ifndef FixedDecimalFormatter_H
#define FixedDecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "DataStruct.d.h"
#include "FixedDecimal.d.h"
#include "FixedDecimalGroupingStrategy.d.h"
#include "Locale.d.h"

#include "FixedDecimalFormatter.d.h"






typedef struct icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1_result {union {FixedDecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1_result;
icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1_result icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1(const DataProvider* provider, const Locale* locale, FixedDecimalGroupingStrategy grouping_strategy);

typedef struct icu4x_FixedDecimalFormatter_create_with_decimal_symbols_v1_mv1_result {union {FixedDecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_FixedDecimalFormatter_create_with_decimal_symbols_v1_mv1_result;
icu4x_FixedDecimalFormatter_create_with_decimal_symbols_v1_mv1_result icu4x_FixedDecimalFormatter_create_with_decimal_symbols_v1_mv1(const DataStruct* data_struct, FixedDecimalGroupingStrategy grouping_strategy);

void icu4x_FixedDecimalFormatter_format_mv1(const FixedDecimalFormatter* self, const FixedDecimal* value, DiplomatWrite* write);


void icu4x_FixedDecimalFormatter_destroy_mv1(FixedDecimalFormatter* self);





#endif // FixedDecimalFormatter_H
