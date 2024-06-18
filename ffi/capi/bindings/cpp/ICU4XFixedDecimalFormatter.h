#ifndef ICU4XFixedDecimalFormatter_H
#define ICU4XFixedDecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"
#include "ICU4XDataProvider.d.h"
#include "ICU4XDataProvider.h"
#include "ICU4XDataStruct.d.h"
#include "ICU4XDataStruct.h"
#include "ICU4XFixedDecimal.d.h"
#include "ICU4XFixedDecimal.h"
#include "ICU4XFixedDecimalGroupingStrategy.d.h"
#include "ICU4XFixedDecimalGroupingStrategy.h"
#include "ICU4XLocale.d.h"
#include "ICU4XLocale.h"
#include "diplomat_result_box_ICU4XFixedDecimalFormatter_ICU4XDataError.d.h"

#include "ICU4XFixedDecimalFormatter.d.h"

#ifdef __cplusplus
namespace capi {
extern "C" {
#endif // __cplusplus


diplomat_result_box_ICU4XFixedDecimalFormatter_ICU4XDataError ICU4XFixedDecimalFormatter_create_with_grouping_strategy(const ICU4XDataProvider* provider, const ICU4XLocale* locale, ICU4XFixedDecimalGroupingStrategy grouping_strategy);

diplomat_result_box_ICU4XFixedDecimalFormatter_ICU4XDataError ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1(const ICU4XDataStruct* data_struct, ICU4XFixedDecimalGroupingStrategy grouping_strategy);

void ICU4XFixedDecimalFormatter_format(const ICU4XFixedDecimalFormatter* self, const ICU4XFixedDecimal* value, DiplomatWrite* write);

void ICU4XFixedDecimalFormatter_destroy(ICU4XFixedDecimalFormatter* self);


#ifdef __cplusplus
} // extern "C"
} // namespace capi
#endif // __cplusplus

#endif // ICU4XFixedDecimalFormatter_H
