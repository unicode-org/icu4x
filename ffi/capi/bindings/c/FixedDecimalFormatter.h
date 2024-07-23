#ifndef FixedDecimalFormatter_H
#define FixedDecimalFormatter_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#include "DataError.d.h"
#include "DataProvider.d.h"
#include "FixedDecimal.d.h"
#include "FixedDecimalGroupingStrategy.d.h"
#include "Locale.d.h"

#include "FixedDecimalFormatter.d.h"






typedef struct icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1_result {union {FixedDecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1_result;
icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1_result icu4x_FixedDecimalFormatter_create_with_grouping_strategy_mv1(const DataProvider* provider, const Locale* locale, FixedDecimalGroupingStrategy grouping_strategy);

typedef struct icu4x_FixedDecimalFormatter_create_with_manual_data_mv1_result {union {FixedDecimalFormatter* ok; DataError err;}; bool is_ok;} icu4x_FixedDecimalFormatter_create_with_manual_data_mv1_result;
icu4x_FixedDecimalFormatter_create_with_manual_data_mv1_result icu4x_FixedDecimalFormatter_create_with_manual_data_mv1(const char* plus_sign_prefix_data, size_t plus_sign_prefix_len, const char* plus_sign_suffix_data, size_t plus_sign_suffix_len, const char* minus_sign_prefix_data, size_t minus_sign_prefix_len, const char* minus_sign_suffix_data, size_t minus_sign_suffix_len, const char* decimal_separator_data, size_t decimal_separator_len, const char* grouping_separator_data, size_t grouping_separator_len, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, const char32_t* digits_data, size_t digits_len, FixedDecimalGroupingStrategy grouping_strategy);

void icu4x_FixedDecimalFormatter_format_mv1(const FixedDecimalFormatter* self, const FixedDecimal* value, DiplomatWrite* write);


void icu4x_FixedDecimalFormatter_destroy_mv1(FixedDecimalFormatter* self);





#endif // FixedDecimalFormatter_H
