#ifndef ICU4XDataStruct_H
#define ICU4XDataStruct_H
#include <stdio.h>
#include <uchar.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include "diplomat_runtime.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct ICU4XDataStruct ICU4XDataStruct;
#include "result_box_ICU4XDataStruct_void.h"

data_struct_ffi_result_box_ICU4XDataStruct_void ICU4XDataStruct_create_decimal_symbols(const char* plus_sign_prefix_data, size_t plus_sign_prefix_len, const char* plus_sign_suffix_data, size_t plus_sign_suffix_len, const char* minus_sign_prefix_data, size_t minus_sign_prefix_len, const char* minus_sign_suffix_data, size_t minus_sign_suffix_len, const char* decimal_separator_data, size_t decimal_separator_len, const char* grouping_separator_data, size_t grouping_separator_len, uint8_t primary_group_size, uint8_t secondary_group_size, uint8_t min_group_size, const char32_t* digits_data, size_t digits_len);
void ICU4XDataStruct_destroy(ICU4XDataStruct* self);

#ifdef __cplusplus
}
#endif
#endif
