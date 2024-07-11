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






typedef struct ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result {union {FixedDecimalFormatter* ok; DataError err;}; bool is_ok;} ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result;
ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result ICU4XFixedDecimalFormatter_create_with_grouping_strategy(const DataProvider* provider, const Locale* locale, FixedDecimalGroupingStrategy grouping_strategy);

typedef struct ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result {union {FixedDecimalFormatter* ok; DataError err;}; bool is_ok;} ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result;
ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1(const DataStruct* data_struct, FixedDecimalGroupingStrategy grouping_strategy);

void ICU4XFixedDecimalFormatter_format(const FixedDecimalFormatter* self, const FixedDecimal* value, DiplomatWrite* write);


void ICU4XFixedDecimalFormatter_destroy(FixedDecimalFormatter* self);





#endif // FixedDecimalFormatter_H
