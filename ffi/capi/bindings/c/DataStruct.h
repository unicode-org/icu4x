#ifndef DataStruct_H
#define DataStruct_H

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"


#include "DataStruct.d.h"






DataStruct* icu4x_DataStruct_create_decimal_symbols_v1_mv1(const char* plus_sign_prefix_data, size_t plus_sign_prefix_len, const char* plus_sign_suffix_data, size_t plus_sign_suffix_len, const char* minus_sign_prefix_data, size_t minus_sign_prefix_len, const char* minus_sign_suffix_data, size_t minus_sign_suffix_len, const char* decimal_separator_data, size_t decimal_separator_len, const char* grouping_separator_data, size_t grouping_separator_len, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, const char32_t* digits_data, size_t digits_len);


void icu4x_DataStruct_destroy_mv1(DataStruct* self);





#endif // DataStruct_H
