#ifndef ICU4XFixedDecimalFormatter_H
#define ICU4XFixedDecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "ICU4XDataError.d.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataStruct.d.h"
#include "ICU4XFixedDecimal.d.h"
#include "ICU4XFixedDecimalGroupingStrategy.d.h"
#include "ICU4XLocale.d.h"

#include "ICU4XFixedDecimalFormatter.d.h"






struct ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result {union {ICU4XFixedDecimalFormatter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XFixedDecimalFormatter_create_with_grouping_strategy_result ICU4XFixedDecimalFormatter_create_with_grouping_strategy(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XFixedDecimalGroupingStrategy grouping_strategy);

struct ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result {union {ICU4XFixedDecimalFormatter* ok; ICU4XDataError err;}; bool is_ok;};
struct ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1_result ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1(const ICU4XDataStruct* data_struct, ICU4XFixedDecimalGroupingStrategy grouping_strategy);

void ICU4XFixedDecimalFormatter_format(const ICU4XFixedDecimalFormatter* self, const ICU4XFixedDecimal* value, DiplomatWrite* write);


void ICU4XFixedDecimalFormatter_destroy(ICU4XFixedDecimalFormatter* self);





#endif // ICU4XFixedDecimalFormatter_H
